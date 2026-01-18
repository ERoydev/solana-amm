use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{LiquidityPoolError, Pool, LIQUIDITY_POOL_SEEDS};

/// # Swap Instruction
///
/// | Step | What Happens                                      |
/// |------|---------------------------------------------------|
/// | 1    | Fee transferred: User → Fee Vault                 |
/// | 2    | Amount after fee: User → Pool Source Escrow       |
/// | 3    | `dy` calculated via AMM formula                   |
/// | 4    | Destination tokens: Pool → User                   |
/// | 5    | Reserves updated based on direction (A→B or B→A)  |
/// | 6    | `last_update` timestamp updated                   |
///
/// ## AMM Formula
/// dy = (y * dx) / (x + dx)
pub fn _swap(ctx: Context<SwapTokens>, amount_source: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let authority = &ctx.accounts.authority;

    let token_program = &ctx.accounts.token_program;
    let fee_vault_token_account = &ctx.accounts.fee_vault_token_account;

    let user_source_token_account = &ctx.accounts.user_source_token_account;
    let user_destination_token_account = &ctx.accounts.user_destination_token_account;

    let pool_escrow_source_token_account = &ctx.accounts.pool_escrow_source_token_account;
    let pool_escrow_destination_token_account = &ctx.accounts.pool_escrow_destination_token_account;

    // Calculate the amount of token to take
    let amount_x = pool_escrow_source_token_account.amount;
    let amount_y = pool_escrow_destination_token_account.amount;

    // Calculate fee and price
    let dx_fee_result = pool.take_fee_amount(amount_source);
    let dy_token_out =
        SwapTokens::get_amount_token(dx_fee_result.amount_after_fee, amount_x, amount_y)?;

    // 1. Transfer fee from user to fee vault
    let transfer_fee_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: user_source_token_account.to_account_info(),
            to: fee_vault_token_account.to_account_info(),
            authority: authority.to_account_info(),
        },
    );
    transfer(transfer_fee_cpi, dx_fee_result.fee_to_take)?;

    // 2. Transfer amount after fee from user to pool escrow
    let transfer_to_pool_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: user_source_token_account.to_account_info(),
            to: pool_escrow_source_token_account.to_account_info(),
            authority: authority.to_account_info(),
        },
    );
    transfer(transfer_to_pool_cpi, dx_fee_result.amount_after_fee)?;

    // 3. Transfer destination tokens from pool to user
    let dy_token_out_u64 = dy_token_out as u64;

    let pool_signer_seeds: &[&[&[u8]]] = &[&[
        LIQUIDITY_POOL_SEEDS.as_bytes(),
        pool.token_a_mint.as_ref(),
        pool.token_b_mint.as_ref(),
        &[pool.bump],
    ]];

    let transfer_destination_to_user_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: pool_escrow_destination_token_account.to_account_info(),
            to: user_destination_token_account.to_account_info(),
            authority: pool.to_account_info(),
        },
    )
    .with_signer(pool_signer_seeds);

    transfer(transfer_destination_to_user_cpi, dy_token_out_u64)?;

    // 4. Update pool reserves based on swap direction
    let is_a_to_b = ctx.accounts.source_token_mint.key() == pool.token_a_mint;

    if is_a_to_b {
        pool.reserve_a = pool
            .reserve_a
            .checked_add(dx_fee_result.amount_after_fee as u128)
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;
        pool.reserve_b = pool
            .reserve_b
            .checked_sub(dy_token_out_u64 as u128)
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;
    } else {
        pool.reserve_b = pool
            .reserve_b
            .checked_add(dx_fee_result.amount_after_fee as u128)
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;
        pool.reserve_a = pool
            .reserve_a
            .checked_sub(dy_token_out_u64 as u128)
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;
    }

    pool.last_update = Clock::get()?.unix_timestamp;

    Ok(())
}

#[derive(Accounts)]
pub struct SwapTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    // Mint accounts
    pub source_token_mint: Account<'info, Mint>,
    pub destination_token_mint: Account<'info, Mint>,

    // User Token Accounts
    #[account(
        mut,
        associated_token::mint = source_token_mint,
        associated_token::authority = authority,
    )]
    pub user_source_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = destination_token_mint,
        associated_token::authority = authority,
    )]
    pub user_destination_token_account: Account<'info, TokenAccount>,

    // Pool Token Accounts
    #[account(
        mut,
        token::mint = source_token_mint,
        token::authority = pool
    )]
    pub pool_escrow_source_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = destination_token_mint,
        token::authority = pool
    )]
    pub pool_escrow_destination_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = source_token_mint,
        token::authority = pool
    )]
    pub fee_vault_token_account: Account<'info, TokenAccount>,

    // System accounts
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> SwapTokens<'info> {
    pub fn get_amount_token(dx_with_fee: u64, amount_x: u64, amount_y: u64) -> Result<u128> {
        // dx -> token to swap
        // dy -> the price of the token out
        // let dy = (amount_y * dx_with_fee) / (amount_x + dx_with_fee);
        let numerator = (amount_y as u128)
            .checked_mul(dx_with_fee as u128)
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;

        let denominator = (amount_x as u128)
            .checked_add(dx_with_fee as u128)
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;

        let dy = numerator
            .checked_div(denominator)
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;

        Ok(dy)
    }
}
