use crate::pool::FeeResult;
use crate::{LiquidityPoolError, LpProvider, Pool, LIQUIDITY_POOL_SEEDS, LP_PROVIDER_SEED};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
};

// Here amount_a and amount_b should be passed in smallest unit representation `10^^9` in my case
pub fn _add_liquidity(
    ctx: Context<AddLiquidity>,
    amount_a_smallest_unit: u64,
    amount_b_smallest_unit: u64,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let provider = &ctx.accounts.provider;

    msg!("Amount A: {}", amount_a_smallest_unit);
    msg!("Amount B: {}", amount_b_smallest_unit);

    // User ATA accounts that holds the tokens
    let user_send_token_a_account_ata = &mut ctx.accounts.user_send_token_a_account_ata;
    let user_send_token_b_account_ata = &ctx.accounts.user_send_token_b_account_ata;

    let escrow_token_a_account = &ctx.accounts.escrow_token_a_account;
    let escrow_token_b_account = &ctx.accounts.escrow_token_b_account;

    let token_program = &ctx.accounts.token_program;

    // 1. Transfer from the creator `ATA` to `Pool token account for tokenA`
    let transfer_token_a_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: user_send_token_a_account_ata.to_account_info(),
            to: escrow_token_a_account.to_account_info(),
            authority: provider.to_account_info(),
        },
    );

    // Take fee from the amount
    let fee_result_token_a: FeeResult = pool.take_fee_amount(amount_a_smallest_unit);

    msg!("FeeResult for Token A: {:?}", fee_result_token_a);
    msg!(
        "Use send token_a_acount: {}",
        user_send_token_a_account_ata.amount
    );

    transfer(transfer_token_a_cpi, fee_result_token_a.amount_after_fee)?;

    // 2. Transfer from the creator `ATA` to `Pool token account for tokenB`
    let transfer_token_b_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: user_send_token_b_account_ata.to_account_info(),
            to: escrow_token_b_account.to_account_info(),
            authority: provider.to_account_info(),
        },
    );

    // Take fee from amount
    let fee_result_token_b: FeeResult = pool.take_fee_amount(amount_b_smallest_unit);

    transfer(transfer_token_b_cpi, fee_result_token_b.amount_after_fee)?;

    msg!("amount a smallest unit: {}", amount_a_smallest_unit);
    msg!("amount b smallest unit: {}", amount_b_smallest_unit);

    let lp_to_mint = if pool.total_lp_supply == 0 {
        AddLiquidity::get_amount_initial_lp_tokens_to_mint(
            fee_result_token_a.amount_after_fee,
            fee_result_token_b.amount_after_fee,
        )
    } else {
        AddLiquidity::get_amount_lp_tokens_to_mint(
            fee_result_token_a.amount_after_fee,
            fee_result_token_b.amount_after_fee,
            pool.reserve_a,
            pool.reserve_b,
            pool.total_lp_supply,
        )
    }?;

    let lp_provider = &mut ctx.accounts.lp_provider;
    lp_provider.pool = pool.key();
    lp_provider.user = provider.key();
    lp_provider.token_a_provided = fee_result_token_a.amount_after_fee;
    lp_provider.token_b_provided = fee_result_token_b.amount_after_fee;
    lp_provider.bump = ctx.bumps.lp_provider;

    let lp_mint = &ctx.accounts.lp_mint;

    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: lp_mint.to_account_info(),
                to: ctx.accounts.lp_user_receive_ata.to_account_info(),
                authority: pool.to_account_info(),
            },
            // Since `Pool` is an PDA i should sign with seeds this transfer
            &[&[
                LIQUIDITY_POOL_SEEDS.as_bytes(),
                pool.token_a_mint.key().as_ref(),
                pool.token_b_mint.key().as_ref(),
                &[pool.bump],
            ]],
        ),
        lp_to_mint,
    )?;

    // 1. Send token_a fees to the pool_fees_vault
    let transfer_fee_token_a_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: user_send_token_a_account_ata.to_account_info(),
            to: ctx.accounts.fee_vault_token_a.to_account_info(),
            authority: provider.to_account_info(),
        },
    );

    msg!("AMOUNT: {}", fee_result_token_b.fee_to_take);

    transfer(transfer_fee_token_a_cpi, fee_result_token_a.fee_to_take)?;

    // 2. Send token_b fees to the pool_fees_vault
    let transfer_fee_token_b_cpi = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: user_send_token_b_account_ata.to_account_info(),
            to: ctx.accounts.fee_vault_token_b.to_account_info(),
            authority: provider.to_account_info(),
        },
    );

    transfer(transfer_fee_token_b_cpi, fee_result_token_b.fee_to_take)?;

    // Update State
    pool.reserve_a = pool.reserve_a.saturating_add(
        fee_result_token_a
            .amount_after_fee
            .try_into()
            .map_err(|_| LiquidityPoolError::InvalidStateUpdateOperation)?,
    );
    pool.reserve_b = pool.reserve_b.saturating_add(
        fee_result_token_b
            .amount_after_fee
            .try_into()
            .map_err(|_| LiquidityPoolError::InvalidStateUpdateOperation)?,
    );

    msg!("Pool: {}", pool.reserve_a);
    lp_provider.lp_tokens_owned = lp_to_mint;
    lp_provider.last_update = Clock::get()?.unix_timestamp;

    Ok(())
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    // These tell my program which token types are being exchanged(USDC, SOL)
    // They are already initialized and user must provide these mint accounts already initialized on Solana.
    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,
    #[account(
        mut,
        mint::decimals = 9,
        mint::authority = pool,
        mint::token_program = token_program,
    )]
    pub lp_mint: Account<'info, Mint>,

    // User ATA Accounts
    #[account(
        mut,
        // Two important constraints in order to tell the Anchor this is an `ATA`, its address derived from owner Pubkey and Mint Pubkey
        associated_token::mint = token_a_mint,
        associated_token::authority = provider,
    )]
    pub user_send_token_a_account_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        // TODO: Adding these constraints, the instruction becomes bigger and i reach `access violation in stack frame`
        // That's why i have commented it, to make the stack usage smaller
        associated_token::mint = token_b_mint,
        associated_token::authority = provider,
    )]
    pub user_send_token_b_account_ata: Account<'info, TokenAccount>,

    // Pool Token Accounts
    #[account(
        mut,
        token::mint = token_a_mint,
        token::authority = pool
    )]
    pub escrow_token_a_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = token_b_mint,
        token::authority = pool
    )]
    pub escrow_token_b_account: Box<Account<'info, TokenAccount>>,

    // LP Accounts
    #[account(
        init,
        payer = provider,
        space = 8 + LpProvider::INIT_SPACE,
        seeds = [LP_PROVIDER_SEED.as_bytes(), provider.key().as_ref(), pool.key().as_ref()],
        bump
    )]
    pub lp_provider: Account<'info, LpProvider>,

    #[account(
        init_if_needed,
        payer = provider,
        associated_token::mint = lp_mint,
        associated_token::authority = provider
    )]
    pub lp_user_receive_ata: Account<'info, TokenAccount>,

    // Fee Accounts
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

impl<'info> AddLiquidity<'info> {
    pub fn get_amount_lp_tokens_to_mint(
        deposit_token_a: u64,
        deposit_token_b: u64,
        reserve_token_a: u128,
        reserve_token_b: u128,
        total_lp_supply: u64,
    ) -> Result<u64> {
        if reserve_token_a == 0 || reserve_token_b == 0 || total_lp_supply == 0 {
            return Ok(0);
        }

        let share_a = (deposit_token_a as u128)
            .checked_mul(total_lp_supply as u128)
            .and_then(|v| v.checked_div(reserve_token_a as u128))
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;

        let share_b = (deposit_token_b as u128)
            .checked_mul(total_lp_supply as u128)
            .and_then(|v| v.checked_div(reserve_token_b))
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;

        Ok(std::cmp::min(share_a, share_b).try_into()?)
    }

    // Used when Pool `Lp token supply` is 0 meaning only for the first liquidity provider
    pub fn get_amount_initial_lp_tokens_to_mint(deposit_a: u64, deposit_b: u64) -> Result<u64> {
        // LP minted = sqrt(token_a_qty * token_b_qty)
        let total: u128 = deposit_a as u128 * deposit_b as u128; // prevent overflow when multiplying
        let amount_lp_to_mint: u128 = AddLiquidity::integer_sqrt(total)?;

        // For testing purposes i use .sqrt(), same results as newton-raphson method
        msg!("Deposit a: {}, Deposit b: {}", deposit_a, deposit_b);
        let lp_tokens_to_mint = (deposit_a as u128)
            .checked_mul(deposit_b as u128)
            .unwrap()
            .isqrt();
        msg!("Lp tokens to mint: {}", lp_tokens_to_mint);
        msg!("Amount lp to mint: {}", amount_lp_to_mint);

        Ok(amount_lp_to_mint.try_into()?)
    }

    // Newton-Raphson method, compute square roots without using floats.
    fn integer_sqrt(value: u128) -> Result<u128> {
        // I have tested this have the same result as using `.isqrt()`
        if value == 0 {
            return Ok(0);
        }

        // Start win an initial guess `x = value`
        // iteratively update the guess until `y >= x` at which point `x` is the integer square root
        let mut x = value;
        // let mut y = (x + value / x) / 2;
        let mut y = value
            .checked_div(x)
            .and_then(|div| div.checked_add(x))
            .and_then(|sum| sum.checked_div(2))
            .ok_or(LiquidityPoolError::InvalidArithmeticOperation)?;

        while y < x {
            x = y;
            y = (x + value / x) / 2;
        }

        Ok(x)
    }
}
