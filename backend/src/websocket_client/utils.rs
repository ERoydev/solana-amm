use std::time::UNIX_EPOCH;
use anchor_lang::AnchorDeserialize;
use base64::{Engine, engine::general_purpose};
use rand::{Rng, rng};
use serde_json::Value;

use crate::websocket_client::{events::TokenInitializedEvent};

pub fn generate_id() -> u32 {
    let random_num = rng().random_range(0..=1000) as u32;

    let now = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        as u32;

    let result = now + random_num;
    result
}

pub fn decode_event(v: Value) -> Option<TokenInitializedEvent> {
    // Navigate to logs array
    if let Some(logs_array) = v["params"]["result"]["value"]["logs"].as_array() {
        if let Some(first_data_log) = logs_array.iter()
            .find_map(|log| log.as_str()
                .and_then(|s| s.strip_prefix("Program data: ")))
        {
            
            let decoded = general_purpose::STANDARD.decode(first_data_log).unwrap();
            let data = &decoded[8..];
            let mut slice: &[u8] = data;
            let event_data = TokenInitializedEvent::deserialize(&mut slice).unwrap();
            return Some(event_data);
        } else {
            return None;
        }
    } else {
        return None;
    }
}