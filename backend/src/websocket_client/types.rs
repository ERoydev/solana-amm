use std::{env, fmt, sync::Arc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::Mutex;
use crate::websocket_client::request::RequestPayload;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EncodingTypes {
    #[serde(rename = "base64")]
    BASE64,
    #[serde(rename = "base58")]
    BASE58,
    #[serde(rename = "jsonParsed")]
    JSONPARSED, // attempts to use program-specific state parsers to return more human-readable and explicit account data.
    #[serde(rename = "base64+zstd")]
    BASE64ZSTD,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CommitmentTypes {
    FINALIZED,
    CONFIRMED,
    PROCESSED,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub commitment: CommitmentTypes,
    pub filters: Option<Vec<serde_json::Value>>,
    pub encoding: Option<EncodingTypes>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            commitment: CommitmentTypes::FINALIZED,
            filters: None, // filters: vec![json!({ "dataSize": 80 })], Example
            encoding: Some(EncodingTypes::BASE64)
        }
    }
}

pub type ProgramSubscribeParamsType = Arc<Mutex<ProgramSubscribeParams>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgramSubscribeParams {
    pub subscription_id: Option<u32>,
    pub params: (Value, Config),
    pub method: String,
}

impl ProgramSubscribeParams {
    pub fn new(method: Methods) -> ProgramSubscribeParamsType {
        let program_id = env::var("PROGRAM_ID").expect("Deployed program id is not set in .env");

        let mut first_param = json!(program_id);
        let config = Config::default();

        match method {
            Methods::LogsSubscribe => {
                let pubkey_to_search_for_in_tx = json!({ "mentions": [program_id] });
                first_param = pubkey_to_search_for_in_tx;
            }
            _ => {}
        }

        let params = (first_param, config);
        let method = method.to_string();

        Arc::new(Mutex::new(ProgramSubscribeParams {
            subscription_id: None,
            params,
            method,
        }))
    }

    pub fn request_payload(&self, id: u32) -> RequestPayload<Value, Config> {
        let request_payload = RequestPayload::new(
            "2.0".to_string(), 
            id, 
            self.method.clone(), 
            self.params.clone()
        );
        request_payload
    }
}

#[derive(Debug)]
pub enum WsEvent {
    Close,
    Error(String),
    // more...
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Methods {
    ProgramSubscribe,
    ProgramUnsubscribe,
    LogsSubscribe,
    LogsUnsubscribe,
    AccountSubscribe,
    AccountUnsubscribe,
    // Add more if needed .. from helius WebSocket API Methods
}


impl fmt::Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Methods::ProgramSubscribe => "programSubscribe",
            Methods::ProgramUnsubscribe => "programUnsubscribe",
            Methods::LogsSubscribe => "logsSubscribe",
            Methods::LogsUnsubscribe => "logsUnsubscribe",
            Methods::AccountSubscribe => "accountSubscribe",
            Methods::AccountUnsubscribe => "accountUnsubscribe",
            // Add more if needed
        };
        write!(f, "{}", s)
    }
}