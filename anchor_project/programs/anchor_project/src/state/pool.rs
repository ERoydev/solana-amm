//! # Pool PDA Accounts
//!
//! The following PDAs are derived and owned by the Pool program for each Pool instance:
//!
//! - **fee_vault_token_a**: TokenAccount PDA for collecting fees in token A
//! - **fee_vault_token_b**: TokenAccount PDA for collecting fees in token B
//! - **lp_provider**: Data Account PDA for each liquidity provider
//! - **escrow_token_a_account**: TokenAccount PDA holding escrowed token A
//! - **escrow_token_b_account**: TokenAccount PDA holding escrowed token B
//! - **lp_mint**: Mint PDA for the pool's LP token
//!
//! > All of these are derived using the Pool's address as a seed.

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub creator: Pubkey,
    // Mint Accounts containing information of the SPL token
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    // Token Account that contains the token from the specific `Mint Account`
    pub escrow_token_a_account: Pubkey,
    pub escrow_token_b_account: Pubkey,
    // The LP_mint of the liquidity pool (The providers keep this as `shares` for their staked tokens)
    pub lp_mint: Pubkey,
    pub total_lp_supply: u64,

    pub fee_bps: u64, // Represents the pool fees basis points, example: 0.30% fee
    pub bump: u8,
    // Holds amount of token currently held in the pool
    pub reserve_a: u128,
    pub reserve_b: u128,
    pub last_update: i64, // const totalLiquidity = (pool.reserve_a * priceA) + (pool.reserve_b * priceB);
}

impl Pool {
    pub fn take_fee_amount(&self, amount_smallest_unit: u64) -> FeeResult {
        let fee_to_take = amount_smallest_unit * self.fee_bps / 100;
        let amount_after_fee = amount_smallest_unit - fee_to_take;
        FeeResult {
            fee_to_take: fee_to_take,
            amount_after_fee: amount_after_fee,
        }
    }
}

// Types
#[derive(Debug)]
pub struct FeeResult {
    pub fee_to_take: u64,
    pub amount_after_fee: u64,
}
