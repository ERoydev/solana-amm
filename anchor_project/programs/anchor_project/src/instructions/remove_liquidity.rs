use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer},
};

use crate::{LiquidityPoolError, LpProvider, Pool, LIQUIDITY_POOL_SEEDS};

/// # Remove Liquidity Instruction
/// Calculate amounts to return based on LP burned 
/// Burn LP tokens from user                       
/// Transfer Token A: Pool Escrow → User           
/// Transfer Token B: Pool Escrow → User           
/// Update pool reserves and LP provider state     
///
/// ## Calculation Formula
/// amount_to_return = (lp_burned / total_lp_supply) * reserve
pub fn _remove_liquidity(ctx: Context<RemoveLiquidity>, lp_amount_to_burn: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let lp_provider = &mut ctx.accounts.lp_provider;
    let provider = &ctx.accounts.provider;

    require!(
        lp_amount_to_burn > 0,
        LiquidityPoolError::InvalidArithmeticOperation
    );
    require!(
        lp_provider.lp_tokens_owned >= lp_amount_to_burn,
        LiquidityPoolError::InsufficientLpTokens
    );
    require!(pool.total_lp_supply > 0, LiquidityPoolError::InvalidArithmeticOperation);

    // 1. Calculate amounts to return
    let (amount_a_to_return, amount_b_to_return) = RemoveLiquidity::calculate_amounts_to_return(
        lp_amount_to_burn,
        pool.total_lp_supply,
        pool.reserve_a,
        pool.reserve_b,
    )?;

    msg!("LP tokens to burn: {}", lp_amount_to_burn);
    msg!("Token A to return: {}", amount_a_to_return);
    msg!("Token B to return: {}", amount_b_to_return);

    let token_program = &ctx.accounts.token_program;

    // 2. Burn LP tokens from user's ATA
    let burn_cpi = CpiContext::new(
        token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.lp_mint.to_account_info(),
            from: ctx.accounts.lp_user_token_account.to_account_info(),
            authority: provider.to_account_info(),
        },
    );
    burn(burn_cpi, lp_amount_to_burn)?;

    // 3. Transfer Token A from pool escrow to user
    let pool_signer_seeds: &[&[&[u8]]] = &[&[
        LIQUIDITY_POOL_SEEDS.as_bytes(),
        pool.token_a_mint.as_ref(),
        pool.token_b_mint.as_ref(),
        &[pool.bump],
    ]];

    let transfer_a_cpi = CpiContext::new_with_signer(
        token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.escrow_token_a_account.to_account_info(),
            to: ctx.accounts.user_receive_token_a_ata.to_account_info(),
            authority: pool.to_account_info(),
        },
        pool_signer_seeds,
    );
    transfer(transfer_a_cpi, amount_a_to_return)?;

    // 4. Transfer Token B from pool escrow to user
    let transfer_b_cpi = CpiContext::new_with_signer(
        token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.escrow_token_b_account.to_account_info(),
            to: ctx.accounts.user_receive_token_b_ata.to_account_info(),
            authority: pool.to_account_info(),
        },
        pool_signer_seeds,
    );
    transfer(transfer_b_cpi, amount_b_to_return)?;

    // 5. Update pool state
    pool.reserve_a = pool
        .reserve_a
        .checked_sub(amount_a_to_return as u128)
        .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;
    pool.reserve_b = pool
        .reserve_b
        .checked_sub(amount_b_to_return as u128)
        .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;
    pool.total_lp_supply = pool
        .total_lp_supply
        .checked_sub(lp_amount_to_burn)
        .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;

    // 6. Update LP provider state
    lp_provider.lp_tokens_owned = lp_provider
        .lp_tokens_owned
        .checked_sub(lp_amount_to_burn)
        .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;
    lp_provider.token_a_provided = lp_provider
        .token_a_provided
        .checked_sub(amount_a_to_return)
        .unwrap_or(0);
    lp_provider.token_b_provided = lp_provider
        .token_b_provided
        .checked_sub(amount_b_to_return)
        .unwrap_or(0);
    lp_provider.last_update = Clock::get()?.unix_timestamp;

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,

    #[account(
        mut,
        mint::authority = pool,
        mint::token_program = token_program,
    )]
    pub lp_mint: Account<'info, Mint>,

    // User LP token account (to burn from)
    #[account(
        mut,
        associated_token::mint = lp_mint,
        associated_token::authority = provider,
    )]
    pub lp_user_token_account: Account<'info, TokenAccount>,

    // User ATAs to receive tokens
    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = provider,
    )]
    pub user_receive_token_a_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = provider,
    )]
    pub user_receive_token_b_ata: Account<'info, TokenAccount>,

    // Pool escrow accounts
    #[account(
        mut,
        token::mint = token_a_mint,
        token::authority = pool
    )]
    pub escrow_token_a_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = token_b_mint,
        token::authority = pool
    )]
    pub escrow_token_b_account: Account<'info, TokenAccount>,

    // LP Provider PDA
    #[account(
        mut,
        seeds = [b"lp-provider", provider.key().as_ref(), pool.key().as_ref()],
        bump = lp_provider.bump,
    )]
    pub lp_provider: Account<'info, LpProvider>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> RemoveLiquidity<'info> {
    /// Calculate how much of each token to return based on LP tokens burned
    ///
    /// Formula: amount_to_return = (lp_burned / total_lp_supply) * reserve
    pub fn calculate_amounts_to_return(
        lp_amount_to_burn: u64,
        total_lp_supply: u64,
        reserve_a: u128,
        reserve_b: u128,
    ) -> Result<(u64, u64)> {
        // amount_a = (lp_burned * reserve_a) / total_lp_supply
        let amount_a = (lp_amount_to_burn as u128)
            .checked_mul(reserve_a)
            .and_then(|v| v.checked_div(total_lp_supply as u128))
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;

        // amount_b = (lp_burned * reserve_b) / total_lp_supply
        let amount_b = (lp_amount_to_burn as u128)
            .checked_mul(reserve_b)
            .and_then(|v| v.checked_div(total_lp_supply as u128))
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;

        Ok((
            amount_a.try_into().map_err(|_| LiquidityPoolError::InvalidArithmeticOperation)?,
            amount_b.try_into().map_err(|_| LiquidityPoolError::InvalidArithmeticOperation)?,
        ))
    }
}
