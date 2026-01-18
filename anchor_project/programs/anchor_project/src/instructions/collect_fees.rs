use anchor_lang::prelude::*;
use anchor_spl::token::transfer;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer},
};

use crate::{LiquidityPoolError, LpProvider, Pool, LIQUIDITY_POOL_SEEDS};

/// # Collect Fees Instruction
///
/// Allows LP providers to collect their proportional share of accumulated fees
///
/// ## Formula
/// user_fees = (user_lp_tokens * total_fees) / total_lp_supply
pub fn _collect_fees(ctx: Context<CollectFees>) -> Result<()> {
    let provider_total_lp_tokens = ctx.accounts.lp_provider.lp_tokens_owned;
    let total_lp_supply = ctx.accounts.lp_mint.supply;
    let total_fees_token_a = ctx.accounts.fee_vault_token_a.amount;
    let total_fees_token_b = ctx.accounts.fee_vault_token_b.amount;

    let token_a_fees_to_collect = CollectFees::get_amount_user_receives(
        provider_total_lp_tokens,
        total_lp_supply,
        total_fees_token_a,
    )?;
    let token_b_fees_to_collect = CollectFees::get_amount_user_receives(
        provider_total_lp_tokens,
        total_lp_supply,
        total_fees_token_b,
    )?;

    let token_program = &ctx.accounts.token_program;
    let token_a_mint = &ctx.accounts.token_a_mint.key();
    let token_b_mint = &ctx.accounts.token_b_mint.key();

    // 1. Send token_a fees to user ATA
    let pool_seeds: &[&[&[u8]]] = &[&[
        LIQUIDITY_POOL_SEEDS.as_bytes(),
        token_a_mint.as_ref(),
        token_b_mint.as_ref(),
        &[ctx.accounts.pool.bump],
    ]];

    let transfer_token_a_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.fee_vault_token_a.to_account_info(),
            to: ctx
                .accounts
                .user_receive_token_a_account_ata
                .to_account_info(),
            authority: ctx.accounts.pool.to_account_info(),
        },
    )
    .with_signer(pool_seeds);

    transfer(transfer_token_a_cpi, token_a_fees_to_collect)?;

    // 2. Send token_b fees to user ATA
    let transfer_token_b_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.fee_vault_token_b.to_account_info(),
            to: ctx
                .accounts
                .user_receive_token_b_account_ata
                .to_account_info(),
            authority: ctx.accounts.pool.to_account_info(),
        },
    )
    .with_signer(pool_seeds);

    transfer(transfer_token_b_cpi, token_b_fees_to_collect)?;

    Ok(())
}

#[derive(Accounts)]
pub struct CollectFees<'info> {
    pub provider: Signer<'info>,
    pub lp_mint: Account<'info, Mint>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(
    mut,
        constraint = lp_provider.pool == pool.key(),
        constraint = lp_provider.user == provider.key()
    )]
    pub lp_provider: Account<'info, LpProvider>,

    // These tell my program which token types are being exchanged(USDC, SOL)
    // They are already initialized and user must provide these mint accounts already initialized on Solana.
    #[account(address = pool.token_a_mint)]
    pub token_a_mint: Account<'info, Mint>,

    #[account(address = pool.token_b_mint)]
    pub token_b_mint: Account<'info, Mint>,

    #[account(
        mut,
        // Two important constraints in order to tell the Anchor this is an `ATA`, its address derived from owner Pubkey and Mint Pubkey
        associated_token::mint = token_a_mint,
        associated_token::authority = provider
    )]
    pub user_receive_token_a_account_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = provider
    )]
    pub user_receive_token_b_account_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub fee_vault_token_a: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = token_b_mint,
        token::authority = pool
    )]
    pub fee_vault_token_b: Box<Account<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> CollectFees<'info> {
    /// Calculate proportional fee share for a user
    /// Formula: user_fees = (user_lp_tokens * total_fees) / total_lp_supply
    pub fn get_amount_user_receives(
        provider_total_lp_tokens: u64,
        total_lp_supply: u64,
        total_fees: u64,
    ) -> Result<u64> {
        if total_lp_supply == 0 {
            return Ok(0);
        }
        
        // Use u128 to prevent overflow during multiplication
        let numerator = (provider_total_lp_tokens as u128)
            .checked_mul(total_fees as u128)
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;
        
        let result = numerator
            .checked_div(total_lp_supply as u128)
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;
        
        result
            .try_into()
            .map_err(|_| LiquidityPoolError::InvalidArithmeticOperation.into())
    }
}
