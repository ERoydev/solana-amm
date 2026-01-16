use crate::websocket_client::utils::decode_event;
use crate::websocket_client::utils::generate_id;

pub use super::response::*;
pub use super::types::*;

use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use rand::{Rng, rng};
use serde_json::Value;
use std::{cmp::min, collections::HashMap, env, sync::Arc};
use tokio::{
    net::TcpStream,
    sync::Mutex,
    time::{Duration, sleep},
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::{connect_async, tungstenite, tungstenite::Message};

// docs: https://www.helius.dev/docs/api-reference/rpc/websocket/programsubscribe#param-config

#[derive(Debug, Clone,)]
pub struct WsConnection {
    pub url: String,
    pub max_retries: u32,
    pub retry_count: u32,
    pub base_delay: u64,
    pub max_delay: u64,
    pub subscriptions: Arc<Mutex<HashMap<u32, Arc<Mutex<ProgramSubscribeParams>>>>>,
    pub is_reconnecting: Arc<Mutex<bool>>,
    pub ws_write: Arc<Mutex<Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>>,
    pub ws_read: Arc<Mutex<Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>>,
    // Channel for receiving notifications from inside of threads
    notify_tx: Option<tokio::sync::mpsc::Sender<WsEvent>>,
}


impl WsConnection {
    pub fn new() -> WsConnection {
        let url = env::var("WEBSOCKET_URL").expect("Websocket ENV is not set in .env");

        WsConnection {
            url,
            max_retries: 10,
            retry_count: 0,
            base_delay: 1000, // Start with 1 second
            max_delay: 30000, // cap at 30 seconds
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            is_reconnecting: Arc::new(Mutex::new(false)),
            ws_write: Arc::new(Mutex::new(None)),
            ws_read: Arc::new(Mutex::new(None)),
            notify_tx: None,
        }
    }

    /// Tries to connect or reconnect if connection cannot be established
    pub async fn connect(&mut self) {
        if *self.is_reconnecting.lock().await {
            return;
        }

        loop {
            match self.try_connect().await {
                Ok(_) => {
                    log::info!("WebSocket connection established");
                    break;
                }
                Err(e) => {
                    // reconnect logic
                    let should_retry = self.schedule_reconnect().await;

                    if !should_retry {
                        log::info!("Max retry attempts has reached. Cannot reconnect: {}!", e);
                        break;
                    }
                }
            }
        }
    }

    /// Will try to establish connection and if successful, it is going to setup event handlers
    pub async fn try_connect(&mut self) -> Result<(), tungstenite::Error> {
        match connect_async(self.url.clone()).await {
            Ok((ws_stream, _)) => {
                
                // Split so i can read and write concurrently
                let (write, read) = ws_stream.split();
                self.ws_write = Arc::new(Mutex::new(Some(write)));
                self.ws_read = Arc::new(Mutex::new(Some(read)));
                
                // setup event handler
                self.setup_event_handlers().await;
                
                // After connection is established trigger resubscribe
                self.on_open().await;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Schedule reconnection, this is going to create delay between reconnection attempts.
    pub async fn schedule_reconnect(&mut self) -> bool {
        if self.retry_count >= self.max_retries {
            log::info!("Max retry attempts reached. Giving up.");
            return false;
        }

        *self.is_reconnecting.lock().await = true;
        self.retry_count += 1;

        // calculate delay exponentially
        let delay = min(
            self.base_delay * 2u64.pow(self.retry_count.saturating_sub(1)),
            self.max_delay,
        );
        
        let random_num = rng().random_range(0..=1000);
        let jittered_delay = delay + random_num; // jitter to prevent multiple clients from retrying at exactly the same time `thundering herd`

        log::info!(
            "Reconnecting in {}ms, (attempt {}/{})",
            jittered_delay,
            self.retry_count,
            self.max_retries
        );

        sleep(Duration::from_millis(jittered_delay)).await;
        return true;
        // self.connect().await;
    }

    /// Spawn task (Thread) to handle incoming messages from the WebSocket
    /// 
    /// This is the so called event handlers 
    pub async fn setup_event_handlers(&self) {
        let ws_read = self.ws_read.clone();
        let subscriptions = self.subscriptions.clone();
        let tx = match &self.notify_tx {
            Some(tx) => tx.clone(),
            None => panic!("Communication channel for events is not set")
        };

        tokio::spawn(async move {
            let mut guard = ws_read.lock().await;

            if let Some(ws_read_stream) = guard.as_mut() {
                while let Some(msg) = ws_read_stream.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            if let Ok(response) =
                                // This should be the response after i have send subscription
                                serde_json::from_str::<RpcResponse<serde_json::Value>>(&text)
                            {
                                log::info!("Received from WebSocket: {:?}", response);
                                let id = response.id;
                                let mut subscription_guard = subscriptions.lock().await;
                                if let Some(param) = subscription_guard.get_mut(&id) {
                                    param.lock().await.subscription_id = Some(id);
                                }
                            } else {
                                match serde_json::from_str::<Value>(&text) {
                                    Ok(received) => {
                                        // TODO: Store in db
                                        let result = decode_event(received).unwrap();
                                        log::info!("Received data from the event is: {:?}", result);
                                    }
                                    Err(e) => {
                                        log::info!("Failed to parse the response: {}", e);
                                    }
                                }
                            }
                        }
                        Ok(Message::Binary(bin)) => {
                            log::info!("Received binary: {} bytes", bin.len());
                        }
                        Ok(Message::Close(_)) => {
                            log::info!("Server closed connection");
                                // Try to send Close event, log if receiver is gone
                                if let Err(e) = tx.send(WsEvent::Close).await {
                                    log::warn!("Failed to send Close event: {}", e);
                                }
                            break;
                        }
                        Ok(Message::Pong(_)) => {}
                        Err(e) => {
                            // For some reason server closed the TCP connection without sending a WebSocket close frame.
                            // I treat this as a normal disconnect and trigger my reconnection logic
                            log::info!("Websocket error:L {:?}", e);
                                if let Err(e) = tx.send(WsEvent::Close).await {
                                    log::warn!("Failed to send Close event: {}", e);
                                }
                            break;
                        }
                        _ => {}
                    }
                }
            }
        });
    }

    /// Registers a new subscription in the internal hashmap and sends the subscription request to the WebSocket server if the connection is open.
    ///
    /// This method ensures that all active subscriptions are tracked and immediately sent when possible.
    pub async fn subscribe(&mut self, subscribe_params: ProgramSubscribeParamsType) {
        let id = generate_id();
        self.subscriptions.lock().await.insert(id, subscribe_params.clone());

        let is_open = {
            let guard = self.ws_write.lock().await;
            guard.is_some() 
        }; // Here in this block scope `guard` goes out of scope and mutex trigger `.unlock()` in the background
  
        if is_open {
            self.send_subscription(subscribe_params, id).await;
        };
    }

    pub async fn re_resubscribe_all(&self) {
        let subscription_guard = self.subscriptions.lock().await;
        log::info!("Restoring {} subscriptions.", subscription_guard.len());

        for (id, subs_params) in subscription_guard.iter() {
            self.send_subscription(subs_params.clone(), id.clone()).await;
        }
    }

    pub async fn send_subscription(&self, subscribe_params: ProgramSubscribeParamsType, id: u32) {
        let payload = subscribe_params.lock().await.request_payload(id);

        match serde_json::to_string(&payload) {
            Ok(request_json) => {
                log::info!("Subscription request send: {}", request_json);
                let mut guard = self.ws_write.lock().await;
                if let Some(write) = guard.as_mut() {
                    if let Err(e) = write.send(Message::Text(request_json)).await {
                        log::info!("Failed to send subscription: {}", e);
                    }
                }
            }
            Err(e) => {
                log::info!("Failed to serialize subscription payload: {}", e);
            }
        }
    }

    // ================== EVENT HANDLERS

    /// Mechanism to pass notifications like (`close`, `error`) from the thread that is reading from WebSocket.
    ///
    /// This allows reconnection to be handled on a single thread, preventing recursive creation of new threads on every reconnect.
    pub async fn event_notification_channel(&mut self) -> tokio::sync::mpsc::Receiver<WsEvent> {
        let (tx, rx) = tokio::sync::mpsc::channel::<WsEvent>(1);

        self.notify_tx = Some(tx);
        return rx;
    }

    pub async fn on_open(&mut self) {
        self.retry_count = 0; // Reset entry count on successful connection
        *self.is_reconnecting.lock().await = false;
        self.re_resubscribe_all().await; // Restore subscriptions, if any
    }

    pub async fn on_close(&mut self) {
        if !*self.is_reconnecting.lock().await {
            self.connect().await; // schedule_reconnect, but i have removed the recursion approach that's why i need to use this instead
        }
    }

    pub async fn on_error(&self, error: &str) {
        log::error!("WebSocket error: {:?}", error);
    }
}