use anchor_lang::prelude::*;

use crate::TokenMetadata;

#[event]
pub struct TokenInitialized {
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub creator: Pubkey,
    pub timestamp: i64,
    pub token_metadata_address: Pubkey,
}

#[event]
pub struct TokensMinted {
    pub mint: Pubkey,
    pub signer: Pubkey,
    pub to_ata: Pubkey,
}
