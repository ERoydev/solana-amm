
mod websocket_client;
use std::sync::Arc;

use tokio::sync::Mutex;
use websocket_client::*;

use crate::ProgramSubscribeParams;

pub async fn listener() {
    let ws_connection = Arc::new(Mutex::new(WsConnection::new()));
    let mut connection_guard = ws_connection.lock().await;

    let mut rx = connection_guard.event_notification_channel().await;
    let ws_connection_clone = ws_connection.clone();

    // Event notification channel receiver, listens here
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            log::info!("Received event: {:?}", event);
            let mut guard = ws_connection_clone.lock().await;
            match event {
                WsEvent::Close => {
                    guard.on_close().await;
                },
                WsEvent::Error(str) => {
                    log::info!("Received event with error message: {}", str);
                    guard.on_error(&str).await;
                }
            }
        }
    });

    connection_guard.connect().await;    
    let subscribe_params = ProgramSubscribeParams::new(Methods::LogsSubscribe);
    connection_guard.subscribe(subscribe_params.clone()).await;


    // Validation that the fields are changed
    // sleep(Duration::from_secs(5)).await; 
    // println!("Ws conne: {:?}", ws_connection.subscriptions);
    // println!("Subscribe Params: {:?}", subscribe_params);
}