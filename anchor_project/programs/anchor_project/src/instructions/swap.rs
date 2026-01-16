use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{LiquidityPoolError, Pool, TEMP_ESCROW_HOLDER};

pub fn _swap(ctx: Context<SwapTokens>, amount_source: u64) -> Result<()> {
    let mut pool = &mut ctx.accounts.pool;
    let authority = &ctx.accounts.authority.key();

    let token_program = &ctx.accounts.token_program;
    let program_escrow_temporary_token_account =
        &ctx.accounts.program_escrow_temporary_token_account;
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
        SwapTokens::get_amount_token(dx_fee_result.amount_after_fee, amount_x, amount_y);

    // 1. Take Source token from user ATA to temporary PDA escrow account
    let transfer_source_from_user_to_temp_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: user_source_token_account.to_account_info(),
            to: program_escrow_temporary_token_account.to_account_info(),
            authority: user_source_token_account.to_account_info(),
        },
    );

    transfer(
        transfer_source_from_user_to_temp_cpi,
        amount_source, // I have to take the full amount from user wallet
    )?;

    let authority_key = authority.key();
    let temporary_escrow_seeds: &[&[&[u8]]] =
        &[&[TEMP_ESCROW_HOLDER.as_bytes(), authority_key.as_ref()]];

    // 2. Send fee to the pool escrow fee holder
    let transfer_fee_from_temp_to_pool_fee_holder_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: program_escrow_temporary_token_account.to_account_info(),
            to: fee_vault_token_account.to_account_info(),
            authority: pool.to_account_info(),
        },
    )
    .with_signer(temporary_escrow_seeds);

    transfer(
        transfer_fee_from_temp_to_pool_fee_holder_cpi,
        dx_fee_result.fee_to_take,
    )?;

    // Send Destination token to user ATA

    // Send fee to Pool Fee Token Account

    // Update States

    Ok(())
}

#[derive(Accounts)]
pub struct SwapTokens<'info> {
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
        seeds = [
            TEMP_ESCROW_HOLDER.as_bytes(),
            authority.key().as_ref()
        ],
        bump
    )]
    pub program_escrow_temporary_token_account: Account<'info, TokenAccount>,

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

/*
dy = (y * dx * (1 - fee)) / (x + dx * (1 - fee))

x = 100 (Token A reserve)
y = 10 (Token B reserve)
dx = 10 (Token A to swap)
fee = 0


dy = (10 * 10) / (100 + 10) = 100 / 110 ≈ 0.909 Token B

*/
