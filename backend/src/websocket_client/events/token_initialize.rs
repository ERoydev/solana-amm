
use anchor_lang::{AnchorDeserialize, prelude::Pubkey};
use anchor_lang::prelude::*;

#[allow(dead_code)]
#[derive(Debug, AnchorDeserialize)]
pub struct TokenInitializedEvent {
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub creator: Pubkey,
    pub timestamp: i64,
    pub token_metadata_address: Pubkey,
}

// Usage
// let program_data = "TUbpfOxczADOnosY8YGJpidftuakIpKwoDwBsVq5Ob6rUkcVrUU7WgQAAABhZGFkBAAAAGRhZGGTXpHuXSGna4fJhUdTxcOFn0nrUioFLW0Nle2Up7+f3qnKHWkAAAAA";
// let decoded = general_purpose::STANDARD.decode(program_data).unwrap();
// let data = &decoded[8..];
// let mut slice: &[u8] = data;
// let event = TokenInitializedEvent::deserialize(&mut slice).unwrap();
// println!("{:?}", event);