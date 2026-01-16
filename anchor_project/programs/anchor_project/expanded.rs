#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use anchor_lang::prelude::*;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
    124u8, 119u8, 4u8, 189u8, 37u8, 47u8, 211u8, 179u8, 149u8, 253u8, 36u8, 136u8, 109u8,
    236u8, 251u8, 192u8, 232u8, 10u8, 232u8, 25u8, 199u8, 43u8, 15u8, 90u8, 35u8, 198u8,
    209u8, 44u8, 197u8, 94u8, 45u8, 86u8,
]);
/// Const version of `ID`
pub const ID_CONST: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
    124u8, 119u8, 4u8, 189u8, 37u8, 47u8, 211u8, 179u8, 149u8, 253u8, 36u8, 136u8, 109u8,
    236u8, 251u8, 192u8, 232u8, 10u8, 232u8, 25u8, 199u8, 43u8, 15u8, 90u8, 35u8, 198u8,
    209u8, 44u8, 197u8, 94u8, 45u8, 86u8,
]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
/// Const version of `ID`
pub const fn id_const() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID_CONST
}
pub mod instructions {
    pub mod initialize_liquidity_pool {
        use anchor_lang::prelude::*;
        use anchor_spl::{
            associated_token::AssociatedToken, token::{Mint, Token, TokenAccount},
        };
        use crate::state::Pool;
        use crate::constants::*;
        pub fn _initialize_liquidity_pool(
            ctx: Context<InitializeLiquidityPool>,
        ) -> Result<()> {
            let pool = &mut ctx.accounts.pool;
            pool.creator = ctx.accounts.creator.key();
            pool.token_a_mint = ctx.accounts.token_a_mint.key();
            pool.token_b_mint = ctx.accounts.token_b_mint.key();
            pool.escrow_token_a_account = ctx.accounts.escrow_token_a_account.key();
            pool.escrow_token_b_account = ctx.accounts.escrow_token_b_account.key();
            pool.lp_mint = ctx.accounts.lp_mint.key();
            pool.total_lp_supply = 0;
            pool.bump = ctx.bumps.pool;
            pool.last_update = Clock::get()?.unix_timestamp;
            pool.fee_bps = POOL_SWAP_FEE;
            pool.reserve_a = 0;
            pool.reserve_b = 0;
            Ok(())
        }
        pub struct InitializeLiquidityPool<'info> {
            #[account(mut)]
            pub creator: Signer<'info>,
            #[account(
                init,
                payer = creator,
                space = 8+Pool::INIT_SPACE,
                seeds = [LIQUIDITY_POOL_SEEDS.as_bytes(),
                token_a_mint.key().as_ref(),
                token_b_mint.key().as_ref()],
                bump
            )]
            pub pool: Account<'info, Pool>,
            #[account(
                init,
                payer = creator,
                mint::decimals = 9,
                mint::authority = pool,
                mint::token_program = token_program,
                seeds = [POOL_LP_MINT_ACCOUNT_SEED.as_bytes(),
                pool.key().as_ref()],
                bump
            )]
            pub lp_mint: Account<'info, Mint>,
            #[account(
                init,
                payer = creator,
                token::mint = token_a_mint,
                token::authority = pool,
                seeds = [ESCROW_A_SEED.as_bytes(),
                pool.key().as_ref()],
                bump
            )]
            pub escrow_token_a_account: Account<'info, TokenAccount>,
            #[account(
                init,
                payer = creator,
                token::mint = token_b_mint,
                token::authority = pool,
                seeds = [ESCROW_B_SEED.as_bytes(),
                pool.key().as_ref()],
                bump
            )]
            pub escrow_token_b_account: Account<'info, TokenAccount>,
            #[account(
                init,
                payer = creator,
                token::mint = token_a_mint,
                token::authority = pool,
                seeds = [FEE_VAULT_TOKEN_A.as_bytes(),
                pool.key().as_ref()],
                bump
            )]
            pub fee_vault_token_a: Account<'info, TokenAccount>,
            #[account(
                init,
                payer = creator,
                token::mint = token_a_mint,
                token::authority = pool,
                seeds = [FEE_VAULT_TOKEN_B.as_bytes(),
                pool.key().as_ref()],
                bump
            )]
            pub fee_vault_token_b: Account<'info, TokenAccount>,
            pub token_a_mint: Account<'info, Mint>,
            pub token_b_mint: Account<'info, Mint>,
            pub token_program: Program<'info, Token>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, InitializeLiquidityPoolBumps>
        for InitializeLiquidityPool<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut InitializeLiquidityPoolBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let creator: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("creator"))?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let pool = &__accounts[0];
                *__accounts = &__accounts[1..];
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let lp_mint = &__accounts[0];
                *__accounts = &__accounts[1..];
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let escrow_token_a_account = &__accounts[0];
                *__accounts = &__accounts[1..];
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let escrow_token_b_account = &__accounts[0];
                *__accounts = &__accounts[1..];
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let fee_vault_token_a = &__accounts[0];
                *__accounts = &__accounts[1..];
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let fee_vault_token_b = &__accounts[0];
                *__accounts = &__accounts[1..];
                let token_a_mint: anchor_lang::accounts::account::Account<Mint> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_a_mint"))?;
                let token_b_mint: anchor_lang::accounts::account::Account<Mint> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_b_mint"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        LIQUIDITY_POOL_SEEDS.as_bytes(),
                        token_a_mint.key().as_ref(),
                        token_b_mint.key().as_ref(),
                    ],
                    __program_id,
                );
                __bumps.pool = __bump;
                if pool.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("pool")
                            .with_pubkeys((pool.key(), __pda_address)),
                    );
                }
                let pool = ({
                    #[inline(never)]
                    || {
                        let actual_field = AsRef::<AccountInfo>::as_ref(&pool);
                        let actual_owner = actual_field.owner;
                        let space = 8 + Pool::INIT_SPACE;
                        let pa: anchor_lang::accounts::account::Account<Pool> = if !false
                            || actual_owner
                                == &anchor_lang::solana_program::system_program::ID
                        {
                            let __current_lamports = pool.lamports();
                            if __current_lamports == 0 {
                                let space = space;
                                let lamports = __anchor_rent.minimum_balance(space);
                                let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                    from: creator.to_account_info(),
                                    to: pool.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::create_account(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    LIQUIDITY_POOL_SEEDS.as_bytes(),
                                                    token_a_mint.key().as_ref(),
                                                    token_b_mint.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    lamports,
                                    space as u64,
                                    __program_id,
                                )?;
                            } else {
                                if creator.key() == pool.key() {
                                    return Err(
                                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .name(),
                                                error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .into(),
                                                error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .to_string(),
                                                error_origin: Some(
                                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/anchor_project/src/instructions/initialize_liquidity_pool.rs",
                                                        line: 40u32,
                                                    }),
                                                ),
                                                compared_values: None,
                                            })
                                            .with_pubkeys((creator.key(), pool.key())),
                                    );
                                }
                                let required_lamports = __anchor_rent
                                    .minimum_balance(space)
                                    .max(1)
                                    .saturating_sub(__current_lamports);
                                if required_lamports > 0 {
                                    let cpi_accounts = anchor_lang::system_program::Transfer {
                                        from: creator.to_account_info(),
                                        to: pool.to_account_info(),
                                    };
                                    let cpi_context = anchor_lang::context::CpiContext::new(
                                        system_program.to_account_info(),
                                        cpi_accounts,
                                    );
                                    anchor_lang::system_program::transfer(
                                        cpi_context,
                                        required_lamports,
                                    )?;
                                }
                                let cpi_accounts = anchor_lang::system_program::Allocate {
                                    account_to_allocate: pool.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::allocate(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    LIQUIDITY_POOL_SEEDS.as_bytes(),
                                                    token_a_mint.key().as_ref(),
                                                    token_b_mint.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    space as u64,
                                )?;
                                let cpi_accounts = anchor_lang::system_program::Assign {
                                    account_to_assign: pool.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::assign(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    LIQUIDITY_POOL_SEEDS.as_bytes(),
                                                    token_a_mint.key().as_ref(),
                                                    token_b_mint.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    __program_id,
                                )?;
                            }
                            match anchor_lang::accounts::account::Account::try_from_unchecked(
                                &pool,
                            ) {
                                Ok(val) => val,
                                Err(e) => return Err(e.with_account_name("pool")),
                            }
                        } else {
                            match anchor_lang::accounts::account::Account::try_from(
                                &pool,
                            ) {
                                Ok(val) => val,
                                Err(e) => return Err(e.with_account_name("pool")),
                            }
                        };
                        if false {
                            if space != actual_field.data_len() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintSpace,
                                        )
                                        .with_account_name("pool")
                                        .with_values((space, actual_field.data_len())),
                                );
                            }
                            if actual_owner != __program_id {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintOwner,
                                        )
                                        .with_account_name("pool")
                                        .with_pubkeys((*actual_owner, *__program_id)),
                                );
                            }
                            {
                                let required_lamports = __anchor_rent
                                    .minimum_balance(space);
                                if pa.to_account_info().lamports() < required_lamports {
                                    return Err(
                                        anchor_lang::error::Error::from(
                                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                            )
                                            .with_account_name("pool"),
                                    );
                                }
                            }
                        }
                        Ok(pa)
                    }
                })()?;
                if !AsRef::<AccountInfo>::as_ref(&pool).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("pool"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        pool.to_account_info().lamports(),
                        pool.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("pool"),
                    );
                }
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[POOL_LP_MINT_ACCOUNT_SEED.as_bytes(), pool.key().as_ref()],
                    __program_id,
                );
                __bumps.lp_mint = __bump;
                if lp_mint.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("lp_mint")
                            .with_pubkeys((lp_mint.key(), __pda_address)),
                    );
                }
                let lp_mint: anchor_lang::accounts::account::Account<Mint> = ({
                    #[inline(never)]
                    || {
                        let owner_program = AsRef::<AccountInfo>::as_ref(&lp_mint).owner;
                        if !false
                            || owner_program
                                == &anchor_lang::solana_program::system_program::ID
                        {
                            let __current_lamports = lp_mint.lamports();
                            if __current_lamports == 0 {
                                let space = ::anchor_spl::token::Mint::LEN;
                                let lamports = __anchor_rent.minimum_balance(space);
                                let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                    from: creator.to_account_info(),
                                    to: lp_mint.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::create_account(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    POOL_LP_MINT_ACCOUNT_SEED.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    lamports,
                                    space as u64,
                                    &token_program.key(),
                                )?;
                            } else {
                                if creator.key() == lp_mint.key() {
                                    return Err(
                                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .name(),
                                                error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .into(),
                                                error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .to_string(),
                                                error_origin: Some(
                                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/anchor_project/src/instructions/initialize_liquidity_pool.rs",
                                                        line: 40u32,
                                                    }),
                                                ),
                                                compared_values: None,
                                            })
                                            .with_pubkeys((creator.key(), lp_mint.key())),
                                    );
                                }
                                let required_lamports = __anchor_rent
                                    .minimum_balance(::anchor_spl::token::Mint::LEN)
                                    .max(1)
                                    .saturating_sub(__current_lamports);
                                if required_lamports > 0 {
                                    let cpi_accounts = anchor_lang::system_program::Transfer {
                                        from: creator.to_account_info(),
                                        to: lp_mint.to_account_info(),
                                    };
                                    let cpi_context = anchor_lang::context::CpiContext::new(
                                        system_program.to_account_info(),
                                        cpi_accounts,
                                    );
                                    anchor_lang::system_program::transfer(
                                        cpi_context,
                                        required_lamports,
                                    )?;
                                }
                                let cpi_accounts = anchor_lang::system_program::Allocate {
                                    account_to_allocate: lp_mint.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::allocate(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    POOL_LP_MINT_ACCOUNT_SEED.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    ::anchor_spl::token::Mint::LEN as u64,
                                )?;
                                let cpi_accounts = anchor_lang::system_program::Assign {
                                    account_to_assign: lp_mint.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::assign(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    POOL_LP_MINT_ACCOUNT_SEED.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    &token_program.key(),
                                )?;
                            }
                            if let Some(extensions) = Option::<
                                &::anchor_spl::token_interface::ExtensionsVec,
                            >::None {
                                for e in extensions {
                                    match e {
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::GroupPointer => {
                                            ::anchor_spl::token_interface::group_pointer_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::GroupPointerInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: lp_mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::GroupMemberPointer => {
                                            ::anchor_spl::token_interface::group_member_pointer_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::GroupMemberPointerInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: lp_mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::MetadataPointer => {
                                            ::anchor_spl::token_interface::metadata_pointer_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::MetadataPointerInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: lp_mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::MintCloseAuthority => {
                                            ::anchor_spl::token_interface::mint_close_authority_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::MintCloseAuthorityInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: lp_mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<&anchor_lang::prelude::Pubkey>::None,
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::TransferHook => {
                                            ::anchor_spl::token_interface::transfer_hook_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::TransferHookInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: lp_mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::NonTransferable => {
                                            ::anchor_spl::token_interface::non_transferable_mint_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::NonTransferableMintInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: lp_mint.to_account_info(),
                                                    },
                                                ),
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::PermanentDelegate => {
                                            ::anchor_spl::token_interface::permanent_delegate_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::PermanentDelegateInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: lp_mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<&anchor_lang::prelude::Pubkey>::None.unwrap(),
                                            )?;
                                        }
                                        _ => {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "not implemented: {0}",
                                                    format_args!("{0:?}", e),
                                                ),
                                            );
                                        }
                                    }
                                }
                            }
                            let cpi_program = token_program.to_account_info();
                            let accounts = ::anchor_spl::token_interface::InitializeMint2 {
                                mint: lp_mint.to_account_info(),
                            };
                            let cpi_ctx = anchor_lang::context::CpiContext::new(
                                cpi_program,
                                accounts,
                            );
                            ::anchor_spl::token_interface::initialize_mint2(
                                cpi_ctx,
                                9,
                                &pool.key(),
                                Option::<&anchor_lang::prelude::Pubkey>::None,
                            )?;
                        }
                        let pa: anchor_lang::accounts::account::Account<Mint> = match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &lp_mint,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("lp_mint")),
                        };
                        if false {
                            if pa.mint_authority
                                != anchor_lang::solana_program::program_option::COption::Some(
                                    pool.key(),
                                )
                            {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintMintMintAuthority,
                                        )
                                        .with_account_name("lp_mint"),
                                );
                            }
                            if pa
                                .freeze_authority
                                .as_ref()
                                .map(|fa| {
                                    Option::<&anchor_lang::prelude::Pubkey>::None
                                        .as_ref()
                                        .map(|expected_fa| fa != *expected_fa)
                                        .unwrap_or(true)
                                })
                                .unwrap_or(
                                    Option::<&anchor_lang::prelude::Pubkey>::None.is_some(),
                                )
                            {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintMintFreezeAuthority,
                                        )
                                        .with_account_name("lp_mint"),
                                );
                            }
                            if pa.decimals != 9 {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintMintDecimals,
                                        )
                                        .with_account_name("lp_mint")
                                        .with_values((pa.decimals, 9)),
                                );
                            }
                            if owner_program != &token_program.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintMintTokenProgram,
                                        )
                                        .with_account_name("lp_mint")
                                        .with_pubkeys((*owner_program, token_program.key())),
                                );
                            }
                        }
                        Ok(pa)
                    }
                })()?;
                if !AsRef::<AccountInfo>::as_ref(&lp_mint).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("lp_mint"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        lp_mint.to_account_info().lamports(),
                        lp_mint.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("lp_mint"),
                    );
                }
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[ESCROW_A_SEED.as_bytes(), pool.key().as_ref()],
                    __program_id,
                );
                __bumps.escrow_token_a_account = __bump;
                if escrow_token_a_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("escrow_token_a_account")
                            .with_pubkeys((escrow_token_a_account.key(), __pda_address)),
                    );
                }
                let escrow_token_a_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = ({
                    #[inline(never)]
                    || {
                        let owner_program = AsRef::<
                            AccountInfo,
                        >::as_ref(&escrow_token_a_account)
                            .owner;
                        if !false
                            || owner_program
                                == &anchor_lang::solana_program::system_program::ID
                        {
                            let __current_lamports = escrow_token_a_account.lamports();
                            if __current_lamports == 0 {
                                let space = {
                                    let mint_info = token_a_mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                };
                                let lamports = __anchor_rent.minimum_balance(space);
                                let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                    from: creator.to_account_info(),
                                    to: escrow_token_a_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::create_account(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    ESCROW_A_SEED.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    lamports,
                                    space as u64,
                                    &token_program.key(),
                                )?;
                            } else {
                                if creator.key() == escrow_token_a_account.key() {
                                    return Err(
                                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .name(),
                                                error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .into(),
                                                error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .to_string(),
                                                error_origin: Some(
                                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/anchor_project/src/instructions/initialize_liquidity_pool.rs",
                                                        line: 40u32,
                                                    }),
                                                ),
                                                compared_values: None,
                                            })
                                            .with_pubkeys((creator.key(), escrow_token_a_account.key())),
                                    );
                                }
                                let required_lamports = __anchor_rent
                                    .minimum_balance({
                                        let mint_info = token_a_mint.to_account_info();
                                        if *mint_info.owner
                                            == ::anchor_spl::token_2022::Token2022::id()
                                        {
                                            use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                                BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                            };
                                            use ::anchor_spl::token_2022::spl_token_2022::state::{
                                                Account, Mint,
                                            };
                                            let mint_data = mint_info.try_borrow_data()?;
                                            let mint_state = StateWithExtensions::<
                                                Mint,
                                            >::unpack(&mint_data)?;
                                            let mint_extensions = mint_state.get_extension_types()?;
                                            let required_extensions = ExtensionType::get_required_init_account_extensions(
                                                &mint_extensions,
                                            );
                                            ExtensionType::try_calculate_account_len::<
                                                Account,
                                            >(&required_extensions)?
                                        } else {
                                            ::anchor_spl::token::TokenAccount::LEN
                                        }
                                    })
                                    .max(1)
                                    .saturating_sub(__current_lamports);
                                if required_lamports > 0 {
                                    let cpi_accounts = anchor_lang::system_program::Transfer {
                                        from: creator.to_account_info(),
                                        to: escrow_token_a_account.to_account_info(),
                                    };
                                    let cpi_context = anchor_lang::context::CpiContext::new(
                                        system_program.to_account_info(),
                                        cpi_accounts,
                                    );
                                    anchor_lang::system_program::transfer(
                                        cpi_context,
                                        required_lamports,
                                    )?;
                                }
                                let cpi_accounts = anchor_lang::system_program::Allocate {
                                    account_to_allocate: escrow_token_a_account
                                        .to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::allocate(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    ESCROW_A_SEED.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    {
                                        let mint_info = token_a_mint.to_account_info();
                                        if *mint_info.owner
                                            == ::anchor_spl::token_2022::Token2022::id()
                                        {
                                            use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                                BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                            };
                                            use ::anchor_spl::token_2022::spl_token_2022::state::{
                                                Account, Mint,
                                            };
                                            let mint_data = mint_info.try_borrow_data()?;
                                            let mint_state = StateWithExtensions::<
                                                Mint,
                                            >::unpack(&mint_data)?;
                                            let mint_extensions = mint_state.get_extension_types()?;
                                            let required_extensions = ExtensionType::get_required_init_account_extensions(
                                                &mint_extensions,
                                            );
                                            ExtensionType::try_calculate_account_len::<
                                                Account,
                                            >(&required_extensions)?
                                        } else {
                                            ::anchor_spl::token::TokenAccount::LEN
                                        }
                                    } as u64,
                                )?;
                                let cpi_accounts = anchor_lang::system_program::Assign {
                                    account_to_assign: escrow_token_a_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::assign(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    ESCROW_A_SEED.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    &token_program.key(),
                                )?;
                            }
                            let cpi_program = token_program.to_account_info();
                            let accounts = ::anchor_spl::token_interface::InitializeAccount3 {
                                account: escrow_token_a_account.to_account_info(),
                                mint: token_a_mint.to_account_info(),
                                authority: pool.to_account_info(),
                            };
                            let cpi_ctx = anchor_lang::context::CpiContext::new(
                                cpi_program,
                                accounts,
                            );
                            ::anchor_spl::token_interface::initialize_account3(cpi_ctx)?;
                        }
                        let pa: anchor_lang::accounts::account::Account<TokenAccount> = match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &escrow_token_a_account,
                        ) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(e.with_account_name("escrow_token_a_account"));
                            }
                        };
                        if false {
                            if pa.mint != token_a_mint.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenMint,
                                        )
                                        .with_account_name("escrow_token_a_account")
                                        .with_pubkeys((pa.mint, token_a_mint.key())),
                                );
                            }
                            if pa.owner != pool.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                                        )
                                        .with_account_name("escrow_token_a_account")
                                        .with_pubkeys((pa.owner, pool.key())),
                                );
                            }
                            if owner_program != &token_program.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenTokenProgram,
                                        )
                                        .with_account_name("escrow_token_a_account")
                                        .with_pubkeys((*owner_program, token_program.key())),
                                );
                            }
                        }
                        Ok(pa)
                    }
                })()?;
                if !AsRef::<AccountInfo>::as_ref(&escrow_token_a_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("escrow_token_a_account"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        escrow_token_a_account.to_account_info().lamports(),
                        escrow_token_a_account.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("escrow_token_a_account"),
                    );
                }
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[ESCROW_B_SEED.as_bytes(), pool.key().as_ref()],
                    __program_id,
                );
                __bumps.escrow_token_b_account = __bump;
                if escrow_token_b_account.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("escrow_token_b_account")
                            .with_pubkeys((escrow_token_b_account.key(), __pda_address)),
                    );
                }
                let escrow_token_b_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = ({
                    #[inline(never)]
                    || {
                        let owner_program = AsRef::<
                            AccountInfo,
                        >::as_ref(&escrow_token_b_account)
                            .owner;
                        if !false
                            || owner_program
                                == &anchor_lang::solana_program::system_program::ID
                        {
                            let __current_lamports = escrow_token_b_account.lamports();
                            if __current_lamports == 0 {
                                let space = {
                                    let mint_info = token_b_mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                };
                                let lamports = __anchor_rent.minimum_balance(space);
                                let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                    from: creator.to_account_info(),
                                    to: escrow_token_b_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::create_account(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    ESCROW_B_SEED.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    lamports,
                                    space as u64,
                                    &token_program.key(),
                                )?;
                            } else {
                                if creator.key() == escrow_token_b_account.key() {
                                    return Err(
                                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .name(),
                                                error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .into(),
                                                error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .to_string(),
                                                error_origin: Some(
                                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/anchor_project/src/instructions/initialize_liquidity_pool.rs",
                                                        line: 40u32,
                                                    }),
                                                ),
                                                compared_values: None,
                                            })
                                            .with_pubkeys((creator.key(), escrow_token_b_account.key())),
                                    );
                                }
                                let required_lamports = __anchor_rent
                                    .minimum_balance({
                                        let mint_info = token_b_mint.to_account_info();
                                        if *mint_info.owner
                                            == ::anchor_spl::token_2022::Token2022::id()
                                        {
                                            use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                                BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                            };
                                            use ::anchor_spl::token_2022::spl_token_2022::state::{
                                                Account, Mint,
                                            };
                                            let mint_data = mint_info.try_borrow_data()?;
                                            let mint_state = StateWithExtensions::<
                                                Mint,
                                            >::unpack(&mint_data)?;
                                            let mint_extensions = mint_state.get_extension_types()?;
                                            let required_extensions = ExtensionType::get_required_init_account_extensions(
                                                &mint_extensions,
                                            );
                                            ExtensionType::try_calculate_account_len::<
                                                Account,
                                            >(&required_extensions)?
                                        } else {
                                            ::anchor_spl::token::TokenAccount::LEN
                                        }
                                    })
                                    .max(1)
                                    .saturating_sub(__current_lamports);
                                if required_lamports > 0 {
                                    let cpi_accounts = anchor_lang::system_program::Transfer {
                                        from: creator.to_account_info(),
                                        to: escrow_token_b_account.to_account_info(),
                                    };
                                    let cpi_context = anchor_lang::context::CpiContext::new(
                                        system_program.to_account_info(),
                                        cpi_accounts,
                                    );
                                    anchor_lang::system_program::transfer(
                                        cpi_context,
                                        required_lamports,
                                    )?;
                                }
                                let cpi_accounts = anchor_lang::system_program::Allocate {
                                    account_to_allocate: escrow_token_b_account
                                        .to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::allocate(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    ESCROW_B_SEED.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    {
                                        let mint_info = token_b_mint.to_account_info();
                                        if *mint_info.owner
                                            == ::anchor_spl::token_2022::Token2022::id()
                                        {
                                            use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                                BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                            };
                                            use ::anchor_spl::token_2022::spl_token_2022::state::{
                                                Account, Mint,
                                            };
                                            let mint_data = mint_info.try_borrow_data()?;
                                            let mint_state = StateWithExtensions::<
                                                Mint,
                                            >::unpack(&mint_data)?;
                                            let mint_extensions = mint_state.get_extension_types()?;
                                            let required_extensions = ExtensionType::get_required_init_account_extensions(
                                                &mint_extensions,
                                            );
                                            ExtensionType::try_calculate_account_len::<
                                                Account,
                                            >(&required_extensions)?
                                        } else {
                                            ::anchor_spl::token::TokenAccount::LEN
                                        }
                                    } as u64,
                                )?;
                                let cpi_accounts = anchor_lang::system_program::Assign {
                                    account_to_assign: escrow_token_b_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::assign(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    ESCROW_B_SEED.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    &token_program.key(),
                                )?;
                            }
                            let cpi_program = token_program.to_account_info();
                            let accounts = ::anchor_spl::token_interface::InitializeAccount3 {
                                account: escrow_token_b_account.to_account_info(),
                                mint: token_b_mint.to_account_info(),
                                authority: pool.to_account_info(),
                            };
                            let cpi_ctx = anchor_lang::context::CpiContext::new(
                                cpi_program,
                                accounts,
                            );
                            ::anchor_spl::token_interface::initialize_account3(cpi_ctx)?;
                        }
                        let pa: anchor_lang::accounts::account::Account<TokenAccount> = match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &escrow_token_b_account,
                        ) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(e.with_account_name("escrow_token_b_account"));
                            }
                        };
                        if false {
                            if pa.mint != token_b_mint.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenMint,
                                        )
                                        .with_account_name("escrow_token_b_account")
                                        .with_pubkeys((pa.mint, token_b_mint.key())),
                                );
                            }
                            if pa.owner != pool.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                                        )
                                        .with_account_name("escrow_token_b_account")
                                        .with_pubkeys((pa.owner, pool.key())),
                                );
                            }
                            if owner_program != &token_program.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenTokenProgram,
                                        )
                                        .with_account_name("escrow_token_b_account")
                                        .with_pubkeys((*owner_program, token_program.key())),
                                );
                            }
                        }
                        Ok(pa)
                    }
                })()?;
                if !AsRef::<AccountInfo>::as_ref(&escrow_token_b_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("escrow_token_b_account"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        escrow_token_b_account.to_account_info().lamports(),
                        escrow_token_b_account.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("escrow_token_b_account"),
                    );
                }
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[FEE_VAULT_TOKEN_A.as_bytes(), pool.key().as_ref()],
                    __program_id,
                );
                __bumps.fee_vault_token_a = __bump;
                if fee_vault_token_a.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("fee_vault_token_a")
                            .with_pubkeys((fee_vault_token_a.key(), __pda_address)),
                    );
                }
                let fee_vault_token_a: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = ({
                    #[inline(never)]
                    || {
                        let owner_program = AsRef::<
                            AccountInfo,
                        >::as_ref(&fee_vault_token_a)
                            .owner;
                        if !false
                            || owner_program
                                == &anchor_lang::solana_program::system_program::ID
                        {
                            let __current_lamports = fee_vault_token_a.lamports();
                            if __current_lamports == 0 {
                                let space = {
                                    let mint_info = token_a_mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                };
                                let lamports = __anchor_rent.minimum_balance(space);
                                let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                    from: creator.to_account_info(),
                                    to: fee_vault_token_a.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::create_account(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    FEE_VAULT_TOKEN_A.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    lamports,
                                    space as u64,
                                    &token_program.key(),
                                )?;
                            } else {
                                if creator.key() == fee_vault_token_a.key() {
                                    return Err(
                                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .name(),
                                                error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .into(),
                                                error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .to_string(),
                                                error_origin: Some(
                                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/anchor_project/src/instructions/initialize_liquidity_pool.rs",
                                                        line: 40u32,
                                                    }),
                                                ),
                                                compared_values: None,
                                            })
                                            .with_pubkeys((creator.key(), fee_vault_token_a.key())),
                                    );
                                }
                                let required_lamports = __anchor_rent
                                    .minimum_balance({
                                        let mint_info = token_a_mint.to_account_info();
                                        if *mint_info.owner
                                            == ::anchor_spl::token_2022::Token2022::id()
                                        {
                                            use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                                BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                            };
                                            use ::anchor_spl::token_2022::spl_token_2022::state::{
                                                Account, Mint,
                                            };
                                            let mint_data = mint_info.try_borrow_data()?;
                                            let mint_state = StateWithExtensions::<
                                                Mint,
                                            >::unpack(&mint_data)?;
                                            let mint_extensions = mint_state.get_extension_types()?;
                                            let required_extensions = ExtensionType::get_required_init_account_extensions(
                                                &mint_extensions,
                                            );
                                            ExtensionType::try_calculate_account_len::<
                                                Account,
                                            >(&required_extensions)?
                                        } else {
                                            ::anchor_spl::token::TokenAccount::LEN
                                        }
                                    })
                                    .max(1)
                                    .saturating_sub(__current_lamports);
                                if required_lamports > 0 {
                                    let cpi_accounts = anchor_lang::system_program::Transfer {
                                        from: creator.to_account_info(),
                                        to: fee_vault_token_a.to_account_info(),
                                    };
                                    let cpi_context = anchor_lang::context::CpiContext::new(
                                        system_program.to_account_info(),
                                        cpi_accounts,
                                    );
                                    anchor_lang::system_program::transfer(
                                        cpi_context,
                                        required_lamports,
                                    )?;
                                }
                                let cpi_accounts = anchor_lang::system_program::Allocate {
                                    account_to_allocate: fee_vault_token_a.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::allocate(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    FEE_VAULT_TOKEN_A.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    {
                                        let mint_info = token_a_mint.to_account_info();
                                        if *mint_info.owner
                                            == ::anchor_spl::token_2022::Token2022::id()
                                        {
                                            use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                                BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                            };
                                            use ::anchor_spl::token_2022::spl_token_2022::state::{
                                                Account, Mint,
                                            };
                                            let mint_data = mint_info.try_borrow_data()?;
                                            let mint_state = StateWithExtensions::<
                                                Mint,
                                            >::unpack(&mint_data)?;
                                            let mint_extensions = mint_state.get_extension_types()?;
                                            let required_extensions = ExtensionType::get_required_init_account_extensions(
                                                &mint_extensions,
                                            );
                                            ExtensionType::try_calculate_account_len::<
                                                Account,
                                            >(&required_extensions)?
                                        } else {
                                            ::anchor_spl::token::TokenAccount::LEN
                                        }
                                    } as u64,
                                )?;
                                let cpi_accounts = anchor_lang::system_program::Assign {
                                    account_to_assign: fee_vault_token_a.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::assign(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    FEE_VAULT_TOKEN_A.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    &token_program.key(),
                                )?;
                            }
                            let cpi_program = token_program.to_account_info();
                            let accounts = ::anchor_spl::token_interface::InitializeAccount3 {
                                account: fee_vault_token_a.to_account_info(),
                                mint: token_a_mint.to_account_info(),
                                authority: pool.to_account_info(),
                            };
                            let cpi_ctx = anchor_lang::context::CpiContext::new(
                                cpi_program,
                                accounts,
                            );
                            ::anchor_spl::token_interface::initialize_account3(cpi_ctx)?;
                        }
                        let pa: anchor_lang::accounts::account::Account<TokenAccount> = match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &fee_vault_token_a,
                        ) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(e.with_account_name("fee_vault_token_a"));
                            }
                        };
                        if false {
                            if pa.mint != token_a_mint.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenMint,
                                        )
                                        .with_account_name("fee_vault_token_a")
                                        .with_pubkeys((pa.mint, token_a_mint.key())),
                                );
                            }
                            if pa.owner != pool.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                                        )
                                        .with_account_name("fee_vault_token_a")
                                        .with_pubkeys((pa.owner, pool.key())),
                                );
                            }
                            if owner_program != &token_program.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenTokenProgram,
                                        )
                                        .with_account_name("fee_vault_token_a")
                                        .with_pubkeys((*owner_program, token_program.key())),
                                );
                            }
                        }
                        Ok(pa)
                    }
                })()?;
                if !AsRef::<AccountInfo>::as_ref(&fee_vault_token_a).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("fee_vault_token_a"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        fee_vault_token_a.to_account_info().lamports(),
                        fee_vault_token_a.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("fee_vault_token_a"),
                    );
                }
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[FEE_VAULT_TOKEN_B.as_bytes(), pool.key().as_ref()],
                    __program_id,
                );
                __bumps.fee_vault_token_b = __bump;
                if fee_vault_token_b.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("fee_vault_token_b")
                            .with_pubkeys((fee_vault_token_b.key(), __pda_address)),
                    );
                }
                let fee_vault_token_b: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = ({
                    #[inline(never)]
                    || {
                        let owner_program = AsRef::<
                            AccountInfo,
                        >::as_ref(&fee_vault_token_b)
                            .owner;
                        if !false
                            || owner_program
                                == &anchor_lang::solana_program::system_program::ID
                        {
                            let __current_lamports = fee_vault_token_b.lamports();
                            if __current_lamports == 0 {
                                let space = {
                                    let mint_info = token_a_mint.to_account_info();
                                    if *mint_info.owner
                                        == ::anchor_spl::token_2022::Token2022::id()
                                    {
                                        use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                            BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                        };
                                        use ::anchor_spl::token_2022::spl_token_2022::state::{
                                            Account, Mint,
                                        };
                                        let mint_data = mint_info.try_borrow_data()?;
                                        let mint_state = StateWithExtensions::<
                                            Mint,
                                        >::unpack(&mint_data)?;
                                        let mint_extensions = mint_state.get_extension_types()?;
                                        let required_extensions = ExtensionType::get_required_init_account_extensions(
                                            &mint_extensions,
                                        );
                                        ExtensionType::try_calculate_account_len::<
                                            Account,
                                        >(&required_extensions)?
                                    } else {
                                        ::anchor_spl::token::TokenAccount::LEN
                                    }
                                };
                                let lamports = __anchor_rent.minimum_balance(space);
                                let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                    from: creator.to_account_info(),
                                    to: fee_vault_token_b.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::create_account(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    FEE_VAULT_TOKEN_B.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    lamports,
                                    space as u64,
                                    &token_program.key(),
                                )?;
                            } else {
                                if creator.key() == fee_vault_token_b.key() {
                                    return Err(
                                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .name(),
                                                error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .into(),
                                                error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .to_string(),
                                                error_origin: Some(
                                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/anchor_project/src/instructions/initialize_liquidity_pool.rs",
                                                        line: 40u32,
                                                    }),
                                                ),
                                                compared_values: None,
                                            })
                                            .with_pubkeys((creator.key(), fee_vault_token_b.key())),
                                    );
                                }
                                let required_lamports = __anchor_rent
                                    .minimum_balance({
                                        let mint_info = token_a_mint.to_account_info();
                                        if *mint_info.owner
                                            == ::anchor_spl::token_2022::Token2022::id()
                                        {
                                            use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                                BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                            };
                                            use ::anchor_spl::token_2022::spl_token_2022::state::{
                                                Account, Mint,
                                            };
                                            let mint_data = mint_info.try_borrow_data()?;
                                            let mint_state = StateWithExtensions::<
                                                Mint,
                                            >::unpack(&mint_data)?;
                                            let mint_extensions = mint_state.get_extension_types()?;
                                            let required_extensions = ExtensionType::get_required_init_account_extensions(
                                                &mint_extensions,
                                            );
                                            ExtensionType::try_calculate_account_len::<
                                                Account,
                                            >(&required_extensions)?
                                        } else {
                                            ::anchor_spl::token::TokenAccount::LEN
                                        }
                                    })
                                    .max(1)
                                    .saturating_sub(__current_lamports);
                                if required_lamports > 0 {
                                    let cpi_accounts = anchor_lang::system_program::Transfer {
                                        from: creator.to_account_info(),
                                        to: fee_vault_token_b.to_account_info(),
                                    };
                                    let cpi_context = anchor_lang::context::CpiContext::new(
                                        system_program.to_account_info(),
                                        cpi_accounts,
                                    );
                                    anchor_lang::system_program::transfer(
                                        cpi_context,
                                        required_lamports,
                                    )?;
                                }
                                let cpi_accounts = anchor_lang::system_program::Allocate {
                                    account_to_allocate: fee_vault_token_b.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::allocate(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    FEE_VAULT_TOKEN_B.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    {
                                        let mint_info = token_a_mint.to_account_info();
                                        if *mint_info.owner
                                            == ::anchor_spl::token_2022::Token2022::id()
                                        {
                                            use ::anchor_spl::token_2022::spl_token_2022::extension::{
                                                BaseStateWithExtensions, ExtensionType, StateWithExtensions,
                                            };
                                            use ::anchor_spl::token_2022::spl_token_2022::state::{
                                                Account, Mint,
                                            };
                                            let mint_data = mint_info.try_borrow_data()?;
                                            let mint_state = StateWithExtensions::<
                                                Mint,
                                            >::unpack(&mint_data)?;
                                            let mint_extensions = mint_state.get_extension_types()?;
                                            let required_extensions = ExtensionType::get_required_init_account_extensions(
                                                &mint_extensions,
                                            );
                                            ExtensionType::try_calculate_account_len::<
                                                Account,
                                            >(&required_extensions)?
                                        } else {
                                            ::anchor_spl::token::TokenAccount::LEN
                                        }
                                    } as u64,
                                )?;
                                let cpi_accounts = anchor_lang::system_program::Assign {
                                    account_to_assign: fee_vault_token_b.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::assign(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    FEE_VAULT_TOKEN_B.as_bytes(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    &token_program.key(),
                                )?;
                            }
                            let cpi_program = token_program.to_account_info();
                            let accounts = ::anchor_spl::token_interface::InitializeAccount3 {
                                account: fee_vault_token_b.to_account_info(),
                                mint: token_a_mint.to_account_info(),
                                authority: pool.to_account_info(),
                            };
                            let cpi_ctx = anchor_lang::context::CpiContext::new(
                                cpi_program,
                                accounts,
                            );
                            ::anchor_spl::token_interface::initialize_account3(cpi_ctx)?;
                        }
                        let pa: anchor_lang::accounts::account::Account<TokenAccount> = match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &fee_vault_token_b,
                        ) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(e.with_account_name("fee_vault_token_b"));
                            }
                        };
                        if false {
                            if pa.mint != token_a_mint.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenMint,
                                        )
                                        .with_account_name("fee_vault_token_b")
                                        .with_pubkeys((pa.mint, token_a_mint.key())),
                                );
                            }
                            if pa.owner != pool.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                                        )
                                        .with_account_name("fee_vault_token_b")
                                        .with_pubkeys((pa.owner, pool.key())),
                                );
                            }
                            if owner_program != &token_program.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenTokenProgram,
                                        )
                                        .with_account_name("fee_vault_token_b")
                                        .with_pubkeys((*owner_program, token_program.key())),
                                );
                            }
                        }
                        Ok(pa)
                    }
                })()?;
                if !AsRef::<AccountInfo>::as_ref(&fee_vault_token_b).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("fee_vault_token_b"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        fee_vault_token_b.to_account_info().lamports(),
                        fee_vault_token_b.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("fee_vault_token_b"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&creator).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("creator"),
                    );
                }
                Ok(InitializeLiquidityPool {
                    creator,
                    pool,
                    lp_mint,
                    escrow_token_a_account,
                    escrow_token_b_account,
                    fee_vault_token_a,
                    fee_vault_token_b,
                    token_a_mint,
                    token_b_mint,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeLiquidityPool<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.creator.to_account_infos());
                account_infos.extend(self.pool.to_account_infos());
                account_infos.extend(self.lp_mint.to_account_infos());
                account_infos.extend(self.escrow_token_a_account.to_account_infos());
                account_infos.extend(self.escrow_token_b_account.to_account_infos());
                account_infos.extend(self.fee_vault_token_a.to_account_infos());
                account_infos.extend(self.fee_vault_token_b.to_account_infos());
                account_infos.extend(self.token_a_mint.to_account_infos());
                account_infos.extend(self.token_b_mint.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeLiquidityPool<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.creator.to_account_metas(None));
                account_metas.extend(self.pool.to_account_metas(None));
                account_metas.extend(self.lp_mint.to_account_metas(None));
                account_metas.extend(self.escrow_token_a_account.to_account_metas(None));
                account_metas.extend(self.escrow_token_b_account.to_account_metas(None));
                account_metas.extend(self.fee_vault_token_a.to_account_metas(None));
                account_metas.extend(self.fee_vault_token_b.to_account_metas(None));
                account_metas.extend(self.token_a_mint.to_account_metas(None));
                account_metas.extend(self.token_b_mint.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitializeLiquidityPool<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.creator, program_id)
                    .map_err(|e| e.with_account_name("creator"))?;
                anchor_lang::AccountsExit::exit(&self.pool, program_id)
                    .map_err(|e| e.with_account_name("pool"))?;
                anchor_lang::AccountsExit::exit(&self.lp_mint, program_id)
                    .map_err(|e| e.with_account_name("lp_mint"))?;
                anchor_lang::AccountsExit::exit(&self.escrow_token_a_account, program_id)
                    .map_err(|e| e.with_account_name("escrow_token_a_account"))?;
                anchor_lang::AccountsExit::exit(&self.escrow_token_b_account, program_id)
                    .map_err(|e| e.with_account_name("escrow_token_b_account"))?;
                anchor_lang::AccountsExit::exit(&self.fee_vault_token_a, program_id)
                    .map_err(|e| e.with_account_name("fee_vault_token_a"))?;
                anchor_lang::AccountsExit::exit(&self.fee_vault_token_b, program_id)
                    .map_err(|e| e.with_account_name("fee_vault_token_b"))?;
                Ok(())
            }
        }
        pub struct InitializeLiquidityPoolBumps {
            pub pool: u8,
            pub lp_mint: u8,
            pub escrow_token_a_account: u8,
            pub escrow_token_b_account: u8,
            pub fee_vault_token_a: u8,
            pub fee_vault_token_b: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InitializeLiquidityPoolBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "pool",
                    "lp_mint",
                    "escrow_token_a_account",
                    "escrow_token_b_account",
                    "fee_vault_token_a",
                    "fee_vault_token_b",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.pool,
                    &self.lp_mint,
                    &self.escrow_token_a_account,
                    &self.escrow_token_b_account,
                    &self.fee_vault_token_a,
                    &&self.fee_vault_token_b,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "InitializeLiquidityPoolBumps",
                    names,
                    values,
                )
            }
        }
        impl Default for InitializeLiquidityPoolBumps {
            fn default() -> Self {
                InitializeLiquidityPoolBumps {
                    pool: u8::MAX,
                    lp_mint: u8::MAX,
                    escrow_token_a_account: u8::MAX,
                    escrow_token_b_account: u8::MAX,
                    fee_vault_token_a: u8::MAX,
                    fee_vault_token_b: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for InitializeLiquidityPool<'info>
        where
            'info: 'info,
        {
            type Bumps = InitializeLiquidityPoolBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_initialize_liquidity_pool {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`InitializeLiquidityPool`].
            pub struct InitializeLiquidityPool {
                pub creator: Pubkey,
                pub pool: Pubkey,
                pub lp_mint: Pubkey,
                pub escrow_token_a_account: Pubkey,
                pub escrow_token_b_account: Pubkey,
                pub fee_vault_token_a: Pubkey,
                pub fee_vault_token_b: Pubkey,
                pub token_a_mint: Pubkey,
                pub token_b_mint: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitializeLiquidityPool
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.creator, writer)?;
                    borsh::BorshSerialize::serialize(&self.pool, writer)?;
                    borsh::BorshSerialize::serialize(&self.lp_mint, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.escrow_token_a_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.escrow_token_b_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.fee_vault_token_a, writer)?;
                    borsh::BorshSerialize::serialize(&self.fee_vault_token_b, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_a_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_b_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitializeLiquidityPool {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.creator,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.pool,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.lp_mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.escrow_token_a_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.escrow_token_b_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.fee_vault_token_a,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.fee_vault_token_b,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_a_mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_b_mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_initialize_liquidity_pool {
            use super::*;
            /// Generated CPI struct of the accounts for [`InitializeLiquidityPool`].
            pub struct InitializeLiquidityPool<'info> {
                pub creator: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub lp_mint: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub escrow_token_a_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub escrow_token_b_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub fee_vault_token_a: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub fee_vault_token_b: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub token_a_mint: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub token_b_mint: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitializeLiquidityPool<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.creator),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.pool),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.lp_mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.escrow_token_a_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.escrow_token_b_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.fee_vault_token_a),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.fee_vault_token_b),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_a_mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_b_mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info>
            for InitializeLiquidityPool<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.creator),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.pool),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.lp_mint),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.escrow_token_a_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.escrow_token_b_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.fee_vault_token_a,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.fee_vault_token_b,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_a_mint,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_b_mint,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
    }
    pub use initialize_liquidity_pool::*;
    pub mod add_liquidity {
        use anchor_lang::prelude::*;
        use anchor_spl::{
            associated_token::AssociatedToken,
            token::{Mint, MintTo, Token, TokenAccount, Transfer, mint_to, transfer},
        };
        use crate::{LIQUIDITY_POOL_SEEDS, LP_PROVIDER_SEED, LpProvider, Pool};
        use crate::pool::FeeResult;
        pub fn _add_liquidity(
            ctx: Context<AddLiquidity>,
            amount_a: u64,
            amount_b: u64,
        ) -> Result<()> {
            let pool = &mut ctx.accounts.pool;
            let provider = &ctx.accounts.provider;
            let user_send_token_a_account_ata = &mut ctx
                .accounts
                .user_send_token_a_account_ata;
            let user_send_token_b_account_ata = &ctx
                .accounts
                .user_send_token_b_account_ata;
            let escrow_token_a_account = &ctx.accounts.escrow_token_a_account;
            let escrow_token_b_account = &ctx.accounts.escrow_token_b_account;
            let token_program = &ctx.accounts.token_program;
            let transfer_token_a_cpi = CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: user_send_token_a_account_ata.to_account_info(),
                    to: escrow_token_a_account.to_account_info(),
                    authority: provider.to_account_info(),
                },
            );
            let fee_result_token_a: FeeResult = pool.take_fee_amount(amount_a);
            transfer(transfer_token_a_cpi, fee_result_token_a.amount_after_fee)?;
            let transfer_token_b_cpi = CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: user_send_token_b_account_ata.to_account_info(),
                    to: escrow_token_b_account.to_account_info(),
                    authority: provider.to_account_info(),
                },
            );
            let fee_result_token_b: FeeResult = pool.take_fee_amount(amount_b);
            transfer(transfer_token_b_cpi, fee_result_token_b.amount_after_fee)?;
            let lp_to_mint = if pool.total_lp_supply == 0 {
                AddLiquidity::get_amount_initial_lp_tokens_to_mint(amount_a, amount_b)
            } else {
                AddLiquidity::get_amount_lp_tokens_to_mint(
                    amount_a,
                    amount_b,
                    pool.reserve_a,
                    pool.reserve_b,
                    pool.total_lp_supply,
                )
            };
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
                    &[
                        &[
                            LIQUIDITY_POOL_SEEDS.as_bytes(),
                            pool.token_a_mint.key().as_ref(),
                            pool.token_b_mint.key().as_ref(),
                            &[pool.bump],
                        ],
                    ],
                ),
                lp_to_mint,
            )?;
            let transfer_fee_token_a_cpi = CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: user_send_token_a_account_ata.to_account_info(),
                    to: ctx.accounts.fee_vault_token_a.to_account_info(),
                    authority: provider.to_account_info(),
                },
            );
            transfer(transfer_fee_token_a_cpi, fee_result_token_a.fee_to_take)?;
            let transfer_fee_token_b_cpi = CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: user_send_token_b_account_ata.to_account_info(),
                    to: ctx.accounts.fee_vault_token_b.to_account_info(),
                    authority: provider.to_account_info(),
                },
            );
            transfer(transfer_fee_token_b_cpi, fee_result_token_b.fee_to_take)?;
            pool.reserve_a += amount_a;
            pool.reserve_b += amount_b;
            pool.last_update = Clock::get()?.unix_timestamp;
            lp_provider.lp_tokens_owned = lp_to_mint;
            lp_provider.last_update = Clock::get()?.unix_timestamp;
            Ok(())
        }
        pub struct AddLiquidity<'info> {
            #[account(mut)]
            pub provider: Signer<'info>,
            #[account(mut)]
            pub pool: Account<'info, Pool>,
            pub token_a_mint: Account<'info, Mint>,
            pub token_b_mint: Account<'info, Mint>,
            #[account(mut)]
            pub user_send_token_a_account_ata: Account<'info, TokenAccount>,
            #[account(mut)]
            pub user_send_token_b_account_ata: Account<'info, TokenAccount>,
            #[account(mut, token::mint = token_a_mint, token::authority = pool)]
            pub escrow_token_a_account: Account<'info, TokenAccount>,
            #[account(mut)]
            pub escrow_token_b_account: Account<'info, TokenAccount>,
            #[account(
                init,
                payer = provider,
                space = 8+LpProvider::INIT_SPACE,
                seeds = [LP_PROVIDER_SEED.as_bytes(),
                provider.key().as_ref(),
                pool.key().as_ref()],
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
            #[account(mut)]
            pub fee_vault_token_a: Account<'info, TokenAccount>,
            #[account(mut)]
            pub fee_vault_token_b: Account<'info, TokenAccount>,
            #[account(
                mut,
                mint::decimals = 9,
                mint::authority = pool,
                mint::token_program = token_program,
            )]
            pub lp_mint: Account<'info, Mint>,
            pub associated_token_program: Program<'info, AssociatedToken>,
            pub token_program: Program<'info, Token>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, AddLiquidityBumps>
        for AddLiquidity<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut AddLiquidityBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let provider: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("provider"))?;
                let pool: anchor_lang::accounts::account::Account<Pool> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("pool"))?;
                let token_a_mint: anchor_lang::accounts::account::Account<Mint> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_a_mint"))?;
                let token_b_mint: anchor_lang::accounts::account::Account<Mint> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_b_mint"))?;
                let user_send_token_a_account_ata: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("user_send_token_a_account_ata"))?;
                let user_send_token_b_account_ata: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("user_send_token_b_account_ata"))?;
                let escrow_token_a_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("escrow_token_a_account"))?;
                let escrow_token_b_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("escrow_token_b_account"))?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let lp_provider = &__accounts[0];
                *__accounts = &__accounts[1..];
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let lp_user_receive_ata = &__accounts[0];
                *__accounts = &__accounts[1..];
                let fee_vault_token_a: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("fee_vault_token_a"))?;
                let fee_vault_token_b: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("fee_vault_token_b"))?;
                let lp_mint: anchor_lang::accounts::account::Account<Mint> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("lp_mint"))?;
                let associated_token_program: anchor_lang::accounts::program::Program<
                    AssociatedToken,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("associated_token_program"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        LP_PROVIDER_SEED.as_bytes(),
                        provider.key().as_ref(),
                        pool.key().as_ref(),
                    ],
                    __program_id,
                );
                __bumps.lp_provider = __bump;
                if lp_provider.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("lp_provider")
                            .with_pubkeys((lp_provider.key(), __pda_address)),
                    );
                }
                let lp_provider = ({
                    #[inline(never)]
                    || {
                        let actual_field = AsRef::<AccountInfo>::as_ref(&lp_provider);
                        let actual_owner = actual_field.owner;
                        let space = 8 + LpProvider::INIT_SPACE;
                        let pa: anchor_lang::accounts::account::Account<LpProvider> = if !false
                            || actual_owner
                                == &anchor_lang::solana_program::system_program::ID
                        {
                            let __current_lamports = lp_provider.lamports();
                            if __current_lamports == 0 {
                                let space = space;
                                let lamports = __anchor_rent.minimum_balance(space);
                                let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                    from: provider.to_account_info(),
                                    to: lp_provider.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::create_account(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    LP_PROVIDER_SEED.as_bytes(),
                                                    provider.key().as_ref(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    lamports,
                                    space as u64,
                                    __program_id,
                                )?;
                            } else {
                                if provider.key() == lp_provider.key() {
                                    return Err(
                                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .name(),
                                                error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .into(),
                                                error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .to_string(),
                                                error_origin: Some(
                                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/anchor_project/src/instructions/add_liquidity.rs",
                                                        line: 135u32,
                                                    }),
                                                ),
                                                compared_values: None,
                                            })
                                            .with_pubkeys((provider.key(), lp_provider.key())),
                                    );
                                }
                                let required_lamports = __anchor_rent
                                    .minimum_balance(space)
                                    .max(1)
                                    .saturating_sub(__current_lamports);
                                if required_lamports > 0 {
                                    let cpi_accounts = anchor_lang::system_program::Transfer {
                                        from: provider.to_account_info(),
                                        to: lp_provider.to_account_info(),
                                    };
                                    let cpi_context = anchor_lang::context::CpiContext::new(
                                        system_program.to_account_info(),
                                        cpi_accounts,
                                    );
                                    anchor_lang::system_program::transfer(
                                        cpi_context,
                                        required_lamports,
                                    )?;
                                }
                                let cpi_accounts = anchor_lang::system_program::Allocate {
                                    account_to_allocate: lp_provider.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::allocate(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    LP_PROVIDER_SEED.as_bytes(),
                                                    provider.key().as_ref(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    space as u64,
                                )?;
                                let cpi_accounts = anchor_lang::system_program::Assign {
                                    account_to_assign: lp_provider.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::assign(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    LP_PROVIDER_SEED.as_bytes(),
                                                    provider.key().as_ref(),
                                                    pool.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    __program_id,
                                )?;
                            }
                            match anchor_lang::accounts::account::Account::try_from_unchecked(
                                &lp_provider,
                            ) {
                                Ok(val) => val,
                                Err(e) => return Err(e.with_account_name("lp_provider")),
                            }
                        } else {
                            match anchor_lang::accounts::account::Account::try_from(
                                &lp_provider,
                            ) {
                                Ok(val) => val,
                                Err(e) => return Err(e.with_account_name("lp_provider")),
                            }
                        };
                        if false {
                            if space != actual_field.data_len() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintSpace,
                                        )
                                        .with_account_name("lp_provider")
                                        .with_values((space, actual_field.data_len())),
                                );
                            }
                            if actual_owner != __program_id {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintOwner,
                                        )
                                        .with_account_name("lp_provider")
                                        .with_pubkeys((*actual_owner, *__program_id)),
                                );
                            }
                            {
                                let required_lamports = __anchor_rent
                                    .minimum_balance(space);
                                if pa.to_account_info().lamports() < required_lamports {
                                    return Err(
                                        anchor_lang::error::Error::from(
                                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                            )
                                            .with_account_name("lp_provider"),
                                    );
                                }
                            }
                        }
                        Ok(pa)
                    }
                })()?;
                if !AsRef::<AccountInfo>::as_ref(&lp_provider).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("lp_provider"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        lp_provider.to_account_info().lamports(),
                        lp_provider.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("lp_provider"),
                    );
                }
                let __anchor_rent = Rent::get()?;
                let lp_user_receive_ata: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = ({
                    #[inline(never)]
                    || {
                        let owner_program = AsRef::<
                            AccountInfo,
                        >::as_ref(&lp_user_receive_ata)
                            .owner;
                        if !true
                            || owner_program
                                == &anchor_lang::solana_program::system_program::ID
                        {
                            ::anchor_spl::associated_token::create(
                                anchor_lang::context::CpiContext::new(
                                    associated_token_program.to_account_info(),
                                    ::anchor_spl::associated_token::Create {
                                        payer: provider.to_account_info(),
                                        associated_token: lp_user_receive_ata.to_account_info(),
                                        authority: provider.to_account_info(),
                                        mint: lp_mint.to_account_info(),
                                        system_program: system_program.to_account_info(),
                                        token_program: token_program.to_account_info(),
                                    },
                                ),
                            )?;
                        }
                        let pa: anchor_lang::accounts::account::Account<TokenAccount> = match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &lp_user_receive_ata,
                        ) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(e.with_account_name("lp_user_receive_ata"));
                            }
                        };
                        if true {
                            if pa.mint != lp_mint.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenMint,
                                        )
                                        .with_account_name("lp_user_receive_ata")
                                        .with_pubkeys((pa.mint, lp_mint.key())),
                                );
                            }
                            if pa.owner != provider.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                                        )
                                        .with_account_name("lp_user_receive_ata")
                                        .with_pubkeys((pa.owner, provider.key())),
                                );
                            }
                            if owner_program != &token_program.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintAssociatedTokenTokenProgram,
                                        )
                                        .with_account_name("lp_user_receive_ata")
                                        .with_pubkeys((*owner_program, token_program.key())),
                                );
                            }
                            if pa.key()
                                != ::anchor_spl::associated_token::get_associated_token_address_with_program_id(
                                    &provider.key(),
                                    &lp_mint.key(),
                                    &token_program.key(),
                                )
                            {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::AccountNotAssociatedTokenAccount,
                                        )
                                        .with_account_name("lp_user_receive_ata"),
                                );
                            }
                        }
                        Ok(pa)
                    }
                })()?;
                if !AsRef::<AccountInfo>::as_ref(&lp_user_receive_ata).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("lp_user_receive_ata"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        lp_user_receive_ata.to_account_info().lamports(),
                        lp_user_receive_ata.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("lp_user_receive_ata"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&provider).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("provider"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&pool).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("pool"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&user_send_token_a_account_ata)
                    .is_writable
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("user_send_token_a_account_ata"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&user_send_token_b_account_ata)
                    .is_writable
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("user_send_token_b_account_ata"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&escrow_token_a_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("escrow_token_a_account"),
                    );
                }
                {
                    if escrow_token_a_account.owner != pool.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner.into(),
                        );
                    }
                    if escrow_token_a_account.mint != token_a_mint.key() {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintTokenMint.into(),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&escrow_token_b_account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("escrow_token_b_account"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&fee_vault_token_a).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("fee_vault_token_a"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&fee_vault_token_b).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("fee_vault_token_b"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&lp_mint).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("lp_mint"),
                    );
                }
                {
                    if lp_mint.decimals != 9 {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintMintDecimals.into(),
                        );
                    }
                    if lp_mint.mint_authority
                        != anchor_lang::solana_program::program_option::COption::Some(
                            pool.key(),
                        )
                    {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintMintMintAuthority
                                .into(),
                        );
                    }
                    if AsRef::<AccountInfo>::as_ref(&lp_mint).owner
                        != &token_program.key()
                    {
                        return Err(
                            anchor_lang::error::ErrorCode::ConstraintMintTokenProgram
                                .into(),
                        );
                    }
                }
                Ok(AddLiquidity {
                    provider,
                    pool,
                    token_a_mint,
                    token_b_mint,
                    user_send_token_a_account_ata,
                    user_send_token_b_account_ata,
                    escrow_token_a_account,
                    escrow_token_b_account,
                    lp_provider,
                    lp_user_receive_ata,
                    fee_vault_token_a,
                    fee_vault_token_b,
                    lp_mint,
                    associated_token_program,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for AddLiquidity<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.provider.to_account_infos());
                account_infos.extend(self.pool.to_account_infos());
                account_infos.extend(self.token_a_mint.to_account_infos());
                account_infos.extend(self.token_b_mint.to_account_infos());
                account_infos
                    .extend(self.user_send_token_a_account_ata.to_account_infos());
                account_infos
                    .extend(self.user_send_token_b_account_ata.to_account_infos());
                account_infos.extend(self.escrow_token_a_account.to_account_infos());
                account_infos.extend(self.escrow_token_b_account.to_account_infos());
                account_infos.extend(self.lp_provider.to_account_infos());
                account_infos.extend(self.lp_user_receive_ata.to_account_infos());
                account_infos.extend(self.fee_vault_token_a.to_account_infos());
                account_infos.extend(self.fee_vault_token_b.to_account_infos());
                account_infos.extend(self.lp_mint.to_account_infos());
                account_infos.extend(self.associated_token_program.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for AddLiquidity<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.provider.to_account_metas(None));
                account_metas.extend(self.pool.to_account_metas(None));
                account_metas.extend(self.token_a_mint.to_account_metas(None));
                account_metas.extend(self.token_b_mint.to_account_metas(None));
                account_metas
                    .extend(self.user_send_token_a_account_ata.to_account_metas(None));
                account_metas
                    .extend(self.user_send_token_b_account_ata.to_account_metas(None));
                account_metas.extend(self.escrow_token_a_account.to_account_metas(None));
                account_metas.extend(self.escrow_token_b_account.to_account_metas(None));
                account_metas.extend(self.lp_provider.to_account_metas(None));
                account_metas.extend(self.lp_user_receive_ata.to_account_metas(None));
                account_metas.extend(self.fee_vault_token_a.to_account_metas(None));
                account_metas.extend(self.fee_vault_token_b.to_account_metas(None));
                account_metas.extend(self.lp_mint.to_account_metas(None));
                account_metas
                    .extend(self.associated_token_program.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for AddLiquidity<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.provider, program_id)
                    .map_err(|e| e.with_account_name("provider"))?;
                anchor_lang::AccountsExit::exit(&self.pool, program_id)
                    .map_err(|e| e.with_account_name("pool"))?;
                anchor_lang::AccountsExit::exit(
                        &self.user_send_token_a_account_ata,
                        program_id,
                    )
                    .map_err(|e| e.with_account_name("user_send_token_a_account_ata"))?;
                anchor_lang::AccountsExit::exit(
                        &self.user_send_token_b_account_ata,
                        program_id,
                    )
                    .map_err(|e| e.with_account_name("user_send_token_b_account_ata"))?;
                anchor_lang::AccountsExit::exit(&self.escrow_token_a_account, program_id)
                    .map_err(|e| e.with_account_name("escrow_token_a_account"))?;
                anchor_lang::AccountsExit::exit(&self.escrow_token_b_account, program_id)
                    .map_err(|e| e.with_account_name("escrow_token_b_account"))?;
                anchor_lang::AccountsExit::exit(&self.lp_provider, program_id)
                    .map_err(|e| e.with_account_name("lp_provider"))?;
                anchor_lang::AccountsExit::exit(&self.lp_user_receive_ata, program_id)
                    .map_err(|e| e.with_account_name("lp_user_receive_ata"))?;
                anchor_lang::AccountsExit::exit(&self.fee_vault_token_a, program_id)
                    .map_err(|e| e.with_account_name("fee_vault_token_a"))?;
                anchor_lang::AccountsExit::exit(&self.fee_vault_token_b, program_id)
                    .map_err(|e| e.with_account_name("fee_vault_token_b"))?;
                anchor_lang::AccountsExit::exit(&self.lp_mint, program_id)
                    .map_err(|e| e.with_account_name("lp_mint"))?;
                Ok(())
            }
        }
        pub struct AddLiquidityBumps {
            pub lp_provider: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AddLiquidityBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AddLiquidityBumps",
                    "lp_provider",
                    &&self.lp_provider,
                )
            }
        }
        impl Default for AddLiquidityBumps {
            fn default() -> Self {
                AddLiquidityBumps {
                    lp_provider: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for AddLiquidity<'info>
        where
            'info: 'info,
        {
            type Bumps = AddLiquidityBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_add_liquidity {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`AddLiquidity`].
            pub struct AddLiquidity {
                pub provider: Pubkey,
                pub pool: Pubkey,
                pub token_a_mint: Pubkey,
                pub token_b_mint: Pubkey,
                pub user_send_token_a_account_ata: Pubkey,
                pub user_send_token_b_account_ata: Pubkey,
                pub escrow_token_a_account: Pubkey,
                pub escrow_token_b_account: Pubkey,
                pub lp_provider: Pubkey,
                pub lp_user_receive_ata: Pubkey,
                pub fee_vault_token_a: Pubkey,
                pub fee_vault_token_b: Pubkey,
                pub lp_mint: Pubkey,
                pub associated_token_program: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for AddLiquidity
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.provider, writer)?;
                    borsh::BorshSerialize::serialize(&self.pool, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_a_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_b_mint, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.user_send_token_a_account_ata,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.user_send_token_b_account_ata,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.escrow_token_a_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.escrow_token_b_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.lp_provider, writer)?;
                    borsh::BorshSerialize::serialize(&self.lp_user_receive_ata, writer)?;
                    borsh::BorshSerialize::serialize(&self.fee_vault_token_a, writer)?;
                    borsh::BorshSerialize::serialize(&self.fee_vault_token_b, writer)?;
                    borsh::BorshSerialize::serialize(&self.lp_mint, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.associated_token_program,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for AddLiquidity {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.provider,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.pool,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_a_mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_b_mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.user_send_token_a_account_ata,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.user_send_token_b_account_ata,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.escrow_token_a_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.escrow_token_b_account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.lp_provider,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.lp_user_receive_ata,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.fee_vault_token_a,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.fee_vault_token_b,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.lp_mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.associated_token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_add_liquidity {
            use super::*;
            /// Generated CPI struct of the accounts for [`AddLiquidity`].
            pub struct AddLiquidity<'info> {
                pub provider: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_a_mint: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub token_b_mint: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub user_send_token_a_account_ata: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub user_send_token_b_account_ata: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub escrow_token_a_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub escrow_token_b_account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub lp_provider: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub lp_user_receive_ata: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub fee_vault_token_a: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub fee_vault_token_b: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub lp_mint: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for AddLiquidity<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.provider),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.pool),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_a_mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_b_mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.user_send_token_a_account_ata),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.user_send_token_b_account_ata),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.escrow_token_a_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.escrow_token_b_account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.lp_provider),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.lp_user_receive_ata),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.fee_vault_token_a),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.fee_vault_token_b),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.lp_mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.associated_token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for AddLiquidity<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.provider),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.pool),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_a_mint,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_b_mint,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.user_send_token_a_account_ata,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.user_send_token_b_account_ata,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.escrow_token_a_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.escrow_token_b_account,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.lp_provider,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.lp_user_receive_ata,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.fee_vault_token_a,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.fee_vault_token_b,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.lp_mint),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.associated_token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        impl<'info> AddLiquidity<'info> {
            pub fn get_amount_lp_tokens_to_mint(
                deposit_token_a: u64,
                deposit_token_b: u64,
                reserve_token_a: u64,
                reserve_token_b: u64,
                total_lp_supply: u64,
            ) -> u64 {
                if reserve_token_a == 0 || reserve_token_b == 0 || total_lp_supply == 0 {
                    return 0;
                }
                let share_a = deposit_token_a as u128 * total_lp_supply as u128
                    / reserve_token_a as u128;
                let share_b = deposit_token_b as u128 * total_lp_supply as u128
                    / reserve_token_b as u128;
                std::cmp::min(share_a, share_b) as u64
            }
            pub fn get_amount_initial_lp_tokens_to_mint(
                deposit_a: u64,
                deposit_b: u64,
            ) -> u64 {
                let total = deposit_a as u128 * deposit_b as u128;
                let amount_lp_to_mint = (total as f64).sqrt() as u64;
                amount_lp_to_mint
            }
        }
    }
    pub use add_liquidity::*;
    pub mod remove_liquidity {
        use anchor_lang::prelude::*;
        pub fn _remove_liquidity(ctx: Context<RemoveLiquidity>) -> Result<()> {
            Ok(())
        }
        pub struct RemoveLiquidity<'info> {
            pub creator: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, RemoveLiquidityBumps>
        for RemoveLiquidity<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut RemoveLiquidityBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let creator: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("creator"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                Ok(RemoveLiquidity {
                    creator,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for RemoveLiquidity<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.creator.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for RemoveLiquidity<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.creator.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for RemoveLiquidity<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                Ok(())
            }
        }
        pub struct RemoveLiquidityBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for RemoveLiquidityBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "RemoveLiquidityBumps")
            }
        }
        impl Default for RemoveLiquidityBumps {
            fn default() -> Self {
                RemoveLiquidityBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for RemoveLiquidity<'info>
        where
            'info: 'info,
        {
            type Bumps = RemoveLiquidityBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_remove_liquidity {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`RemoveLiquidity`].
            pub struct RemoveLiquidity {
                pub creator: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for RemoveLiquidity
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.creator, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for RemoveLiquidity {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.creator,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_remove_liquidity {
            use super::*;
            /// Generated CPI struct of the accounts for [`RemoveLiquidity`].
            pub struct RemoveLiquidity<'info> {
                pub creator: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for RemoveLiquidity<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.creator),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for RemoveLiquidity<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.creator),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        impl<'info> RemoveLiquidity<'info> {
            pub fn get_amount_lp_tokens_to_burn() -> u64 {
                ::core::panicking::panic("not yet implemented");
            }
        }
    }
    pub use remove_liquidity::*;
    pub mod swap {
        use anchor_lang::prelude::*;
        pub fn _swap(ctx: Context<SwapTokens>) -> Result<()> {
            Ok(())
        }
        pub struct SwapTokens<'info> {
            pub creator: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, SwapTokensBumps> for SwapTokens<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut SwapTokensBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let creator: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("creator"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                Ok(SwapTokens {
                    creator,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for SwapTokens<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.creator.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for SwapTokens<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.creator.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for SwapTokens<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                Ok(())
            }
        }
        pub struct SwapTokensBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for SwapTokensBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "SwapTokensBumps")
            }
        }
        impl Default for SwapTokensBumps {
            fn default() -> Self {
                SwapTokensBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for SwapTokens<'info>
        where
            'info: 'info,
        {
            type Bumps = SwapTokensBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_swap_tokens {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`SwapTokens`].
            pub struct SwapTokens {
                pub creator: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for SwapTokens
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.creator, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for SwapTokens {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.creator,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_swap_tokens {
            use super::*;
            /// Generated CPI struct of the accounts for [`SwapTokens`].
            pub struct SwapTokens<'info> {
                pub creator: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for SwapTokens<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.creator),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for SwapTokens<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.creator),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        impl<'info> SwapTokens<'info> {
            pub fn get_amount_token() -> u64 {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub use swap::*;
    pub mod collect_fees {
        use anchor_lang::prelude::*;
        use anchor_spl::{
            associated_token::AssociatedToken,
            token::{Mint, Token, TokenAccount, Transfer},
        };
        use anchor_spl::token::transfer;
        use crate::{LIQUIDITY_POOL_SEEDS, LpProvider, Pool};
        pub fn _collect_fees(ctx: Context<CollectFees>) -> Result<()> {
            let provider_total_lp_tokens = ctx.accounts.lp_provider.lp_tokens_owned;
            let total_lp_supply = ctx.accounts.lp_mint.supply;
            let token_program = &ctx.accounts.token_program;
            let token_a_mint = &ctx.accounts.token_a_mint.key();
            let token_b_mint = &ctx.accounts.token_b_mint.key();
            let pool_seeds: &[&[&[u8]]] = &[
                &[
                    LIQUIDITY_POOL_SEEDS.as_bytes(),
                    token_a_mint.as_ref(),
                    token_b_mint.as_ref(),
                    &[ctx.accounts.pool.bump],
                ],
            ];
            Ok(())
        }
        pub struct CollectFees<'info> {
            pub provider: Signer<'info>,
            pub lp_mint: Account<'info, Mint>,
            #[account(mut)]
            pub pool: Account<'info, Pool>,
            #[account(
                mut,
                constraint = lp_provider.pool = = pool.key(),
                constraint = lp_provider.user = = provider.key()
            )]
            pub lp_provider: Account<'info, LpProvider>,
            #[account(address = pool.token_a_mint)]
            pub token_a_mint: Account<'info, Mint>,
            #[account(address = pool.token_b_mint)]
            pub token_b_mint: Account<'info, Mint>,
            #[account(
                mut,
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
            pub associated_token_program: Program<'info, AssociatedToken>,
            pub token_program: Program<'info, Token>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, CollectFeesBumps> for CollectFees<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut CollectFeesBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let provider: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("provider"))?;
                let lp_mint: anchor_lang::accounts::account::Account<Mint> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("lp_mint"))?;
                let pool: anchor_lang::accounts::account::Account<Pool> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("pool"))?;
                let lp_provider: anchor_lang::accounts::account::Account<LpProvider> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("lp_provider"))?;
                let token_a_mint: anchor_lang::accounts::account::Account<Mint> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_a_mint"))?;
                let token_b_mint: anchor_lang::accounts::account::Account<Mint> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_b_mint"))?;
                let user_receive_token_a_account_ata: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| {
                        e.with_account_name("user_receive_token_a_account_ata")
                    })?;
                let user_receive_token_b_account_ata: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| {
                        e.with_account_name("user_receive_token_b_account_ata")
                    })?;
                let associated_token_program: anchor_lang::accounts::program::Program<
                    AssociatedToken,
                > = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("associated_token_program"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if !AsRef::<AccountInfo>::as_ref(&pool).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("pool"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&lp_provider).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("lp_provider"),
                    );
                }
                if !(lp_provider.pool == pool.key()) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("lp_provider"),
                    );
                }
                if !(lp_provider.user == provider.key()) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("lp_provider"),
                    );
                }
                {
                    let actual = token_a_mint.key();
                    let expected = pool.token_a_mint;
                    if actual != expected {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintAddress,
                                )
                                .with_account_name("token_a_mint")
                                .with_pubkeys((actual, expected)),
                        );
                    }
                }
                {
                    let actual = token_b_mint.key();
                    let expected = pool.token_b_mint;
                    if actual != expected {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintAddress,
                                )
                                .with_account_name("token_b_mint")
                                .with_pubkeys((actual, expected)),
                        );
                    }
                }
                {
                    let my_owner = user_receive_token_a_account_ata.owner;
                    let wallet_address = provider.key();
                    if my_owner != wallet_address {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                                )
                                .with_account_name("user_receive_token_a_account_ata")
                                .with_pubkeys((my_owner, wallet_address)),
                        );
                    }
                    let __associated_token_address = ::anchor_spl::associated_token::get_associated_token_address(
                        &wallet_address,
                        &token_a_mint.key(),
                    );
                    let my_key = user_receive_token_a_account_ata.key();
                    if my_key != __associated_token_address {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintAssociated,
                                )
                                .with_account_name("user_receive_token_a_account_ata")
                                .with_pubkeys((my_key, __associated_token_address)),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&user_receive_token_a_account_ata)
                    .is_writable
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("user_receive_token_a_account_ata"),
                    );
                }
                {
                    let my_owner = user_receive_token_b_account_ata.owner;
                    let wallet_address = provider.key();
                    if my_owner != wallet_address {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                                )
                                .with_account_name("user_receive_token_b_account_ata")
                                .with_pubkeys((my_owner, wallet_address)),
                        );
                    }
                    let __associated_token_address = ::anchor_spl::associated_token::get_associated_token_address(
                        &wallet_address,
                        &token_b_mint.key(),
                    );
                    let my_key = user_receive_token_b_account_ata.key();
                    if my_key != __associated_token_address {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintAssociated,
                                )
                                .with_account_name("user_receive_token_b_account_ata")
                                .with_pubkeys((my_key, __associated_token_address)),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&user_receive_token_b_account_ata)
                    .is_writable
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("user_receive_token_b_account_ata"),
                    );
                }
                Ok(CollectFees {
                    provider,
                    lp_mint,
                    pool,
                    lp_provider,
                    token_a_mint,
                    token_b_mint,
                    user_receive_token_a_account_ata,
                    user_receive_token_b_account_ata,
                    associated_token_program,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CollectFees<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.provider.to_account_infos());
                account_infos.extend(self.lp_mint.to_account_infos());
                account_infos.extend(self.pool.to_account_infos());
                account_infos.extend(self.lp_provider.to_account_infos());
                account_infos.extend(self.token_a_mint.to_account_infos());
                account_infos.extend(self.token_b_mint.to_account_infos());
                account_infos
                    .extend(self.user_receive_token_a_account_ata.to_account_infos());
                account_infos
                    .extend(self.user_receive_token_b_account_ata.to_account_infos());
                account_infos.extend(self.associated_token_program.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CollectFees<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.provider.to_account_metas(None));
                account_metas.extend(self.lp_mint.to_account_metas(None));
                account_metas.extend(self.pool.to_account_metas(None));
                account_metas.extend(self.lp_provider.to_account_metas(None));
                account_metas.extend(self.token_a_mint.to_account_metas(None));
                account_metas.extend(self.token_b_mint.to_account_metas(None));
                account_metas
                    .extend(
                        self.user_receive_token_a_account_ata.to_account_metas(None),
                    );
                account_metas
                    .extend(
                        self.user_receive_token_b_account_ata.to_account_metas(None),
                    );
                account_metas
                    .extend(self.associated_token_program.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for CollectFees<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.pool, program_id)
                    .map_err(|e| e.with_account_name("pool"))?;
                anchor_lang::AccountsExit::exit(&self.lp_provider, program_id)
                    .map_err(|e| e.with_account_name("lp_provider"))?;
                anchor_lang::AccountsExit::exit(
                        &self.user_receive_token_a_account_ata,
                        program_id,
                    )
                    .map_err(|e| {
                        e.with_account_name("user_receive_token_a_account_ata")
                    })?;
                anchor_lang::AccountsExit::exit(
                        &self.user_receive_token_b_account_ata,
                        program_id,
                    )
                    .map_err(|e| {
                        e.with_account_name("user_receive_token_b_account_ata")
                    })?;
                Ok(())
            }
        }
        pub struct CollectFeesBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for CollectFeesBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "CollectFeesBumps")
            }
        }
        impl Default for CollectFeesBumps {
            fn default() -> Self {
                CollectFeesBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for CollectFees<'info>
        where
            'info: 'info,
        {
            type Bumps = CollectFeesBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_collect_fees {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`CollectFees`].
            pub struct CollectFees {
                pub provider: Pubkey,
                pub lp_mint: Pubkey,
                pub pool: Pubkey,
                pub lp_provider: Pubkey,
                pub token_a_mint: Pubkey,
                pub token_b_mint: Pubkey,
                pub user_receive_token_a_account_ata: Pubkey,
                pub user_receive_token_b_account_ata: Pubkey,
                pub associated_token_program: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for CollectFees
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.provider, writer)?;
                    borsh::BorshSerialize::serialize(&self.lp_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.pool, writer)?;
                    borsh::BorshSerialize::serialize(&self.lp_provider, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_a_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_b_mint, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.user_receive_token_a_account_ata,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.user_receive_token_b_account_ata,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.associated_token_program,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for CollectFees {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.provider,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.lp_mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.pool,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.lp_provider,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_a_mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_b_mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.user_receive_token_a_account_ata,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.user_receive_token_b_account_ata,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.associated_token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_collect_fees {
            use super::*;
            /// Generated CPI struct of the accounts for [`CollectFees`].
            pub struct CollectFees<'info> {
                pub provider: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub lp_mint: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub lp_provider: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub token_a_mint: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub token_b_mint: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub user_receive_token_a_account_ata: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub user_receive_token_b_account_ata: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for CollectFees<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.provider),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.lp_mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.pool),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.lp_provider),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_a_mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_b_mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(
                                    &self.user_receive_token_a_account_ata,
                                ),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(
                                    &self.user_receive_token_b_account_ata,
                                ),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.associated_token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for CollectFees<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.provider),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.lp_mint),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.pool),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.lp_provider,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_a_mint,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_b_mint,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.user_receive_token_a_account_ata,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.user_receive_token_b_account_ata,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.associated_token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        impl<'info> CollectFees<'info> {
            pub fn get_amount_user_receives(
                provider_total_lp_tokens: u64,
                total_lp_supply: u64,
                total_fees: u64,
            ) -> u64 {
                if total_lp_supply == 0 {
                    return 0;
                }
                let user_share = provider_total_lp_tokens as f64
                    / total_lp_supply as f64;
                (user_share * total_fees as f64) as u64
            }
        }
    }
    pub use collect_fees::*;
}
pub mod constants {
    use anchor_lang::prelude::*;
    pub const POOL_SWAP_FEE: f64 = 0.03;
    pub const POOL_LP_MINT_ACCOUNT_SEED: &str = "lp-mint";
    pub const POOL_FEES_VAULT_SEED: &str = "pool-fees-vault";
    pub const LP_PROVIDER_SEED: &str = "lp-provider";
    pub const ESCROW_A_SEED: &str = "escrow-a";
    pub const ESCROW_B_SEED: &str = "escrow-b";
    pub const FEE_VAULT_TOKEN_A: &str = "fee-vault-token-a";
    pub const FEE_VAULT_TOKEN_B: &str = "fee-vault-token-b";
    pub const LIQUIDITY_POOL_SEEDS: &str = "liquidity_pool";
    pub const TOKEN_METADATA_SEED: &str = "token_metadata";
}
pub mod errors {
    use anchor_lang::prelude::*;
    #[repr(u32)]
    pub enum LiquidityPoolError {
        InvalidPool,
        InvalidMint,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LiquidityPoolError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    LiquidityPoolError::InvalidPool => "InvalidPool",
                    LiquidityPoolError::InvalidMint => "InvalidMint",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for LiquidityPoolError {
        #[inline]
        fn clone(&self) -> LiquidityPoolError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for LiquidityPoolError {}
    impl LiquidityPoolError {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                LiquidityPoolError::InvalidPool => "InvalidPool".to_string(),
                LiquidityPoolError::InvalidMint => "InvalidMint".to_string(),
            }
        }
    }
    impl From<LiquidityPoolError> for u32 {
        fn from(e: LiquidityPoolError) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<LiquidityPoolError> for anchor_lang::error::Error {
        fn from(error_code: LiquidityPoolError) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for LiquidityPoolError {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                LiquidityPoolError::InvalidPool => {
                    fmt.write_fmt(format_args!("Invalid pool"))
                }
                LiquidityPoolError::InvalidMint => {
                    fmt.write_fmt(
                        format_args!("Lp tokens to mint should be greater than zero"),
                    )
                }
            }
        }
    }
}
pub mod state {
    pub mod pool {
        use anchor_lang::prelude::*;
        pub struct Pool {
            pub creator: Pubkey,
            pub token_a_mint: Pubkey,
            pub token_b_mint: Pubkey,
            pub escrow_token_a_account: Pubkey,
            pub escrow_token_b_account: Pubkey,
            pub lp_mint: Pubkey,
            pub total_lp_supply: u64,
            pub fee_bps: f64,
            pub bump: u8,
            pub reserve_a: u64,
            pub reserve_b: u64,
            pub last_update: i64,
        }
        #[automatically_derived]
        impl anchor_lang::Space for Pool {
            const INIT_SPACE: usize = 0 + 32 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 1 + 8 + 8
                + 8;
        }
        impl borsh::ser::BorshSerialize for Pool
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            f64: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.creator, writer)?;
                borsh::BorshSerialize::serialize(&self.token_a_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.token_b_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.escrow_token_a_account, writer)?;
                borsh::BorshSerialize::serialize(&self.escrow_token_b_account, writer)?;
                borsh::BorshSerialize::serialize(&self.lp_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.total_lp_supply, writer)?;
                borsh::BorshSerialize::serialize(&self.fee_bps, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                borsh::BorshSerialize::serialize(&self.reserve_a, writer)?;
                borsh::BorshSerialize::serialize(&self.reserve_b, writer)?;
                borsh::BorshSerialize::serialize(&self.last_update, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Pool
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            f64: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    creator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    token_a_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    token_b_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    escrow_token_a_account: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    escrow_token_b_account: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    lp_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    total_lp_supply: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    fee_bps: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    reserve_a: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    reserve_b: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    last_update: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Pool {
            #[inline]
            fn clone(&self) -> Pool {
                Pool {
                    creator: ::core::clone::Clone::clone(&self.creator),
                    token_a_mint: ::core::clone::Clone::clone(&self.token_a_mint),
                    token_b_mint: ::core::clone::Clone::clone(&self.token_b_mint),
                    escrow_token_a_account: ::core::clone::Clone::clone(
                        &self.escrow_token_a_account,
                    ),
                    escrow_token_b_account: ::core::clone::Clone::clone(
                        &self.escrow_token_b_account,
                    ),
                    lp_mint: ::core::clone::Clone::clone(&self.lp_mint),
                    total_lp_supply: ::core::clone::Clone::clone(&self.total_lp_supply),
                    fee_bps: ::core::clone::Clone::clone(&self.fee_bps),
                    bump: ::core::clone::Clone::clone(&self.bump),
                    reserve_a: ::core::clone::Clone::clone(&self.reserve_a),
                    reserve_b: ::core::clone::Clone::clone(&self.reserve_b),
                    last_update: ::core::clone::Clone::clone(&self.last_update),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for Pool {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(Pool::DISCRIMINATOR).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for Pool {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < Pool::DISCRIMINATOR.len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..Pool::DISCRIMINATOR.len()];
                if Pool::DISCRIMINATOR != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/anchor_project/src/state/pool.rs",
                                        line: 3u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("Pool"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[Pool::DISCRIMINATOR.len()..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for Pool {
            const DISCRIMINATOR: &'static [u8] = &[241, 154, 109, 4, 17, 177, 109, 188];
        }
        #[automatically_derived]
        impl anchor_lang::Owner for Pool {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        impl Pool {
            pub fn take_fee_amount(&self, amount: u64) -> FeeResult {
                let fee_to_take = (amount as f64 * self.fee_bps) as u64;
                let amount_after_fee = amount - fee_to_take;
                FeeResult {
                    fee_to_take,
                    amount_after_fee,
                }
            }
        }
        pub struct FeeResult {
            pub fee_to_take: u64,
            pub amount_after_fee: u64,
        }
    }
    pub use pool::*;
    pub mod lp_provider {
        use anchor_lang::prelude::*;
        pub struct LpProvider {
            pub pool: Pubkey,
            pub user: Pubkey,
            pub token_a_provided: u64,
            pub token_b_provided: u64,
            pub lp_tokens_owned: u64,
            pub last_update: i64,
            pub bump: u8,
        }
        #[automatically_derived]
        impl anchor_lang::Space for LpProvider {
            const INIT_SPACE: usize = 0 + 32 + 32 + 8 + 8 + 8 + 8 + 1;
        }
        impl borsh::ser::BorshSerialize for LpProvider
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.user, writer)?;
                borsh::BorshSerialize::serialize(&self.token_a_provided, writer)?;
                borsh::BorshSerialize::serialize(&self.token_b_provided, writer)?;
                borsh::BorshSerialize::serialize(&self.lp_tokens_owned, writer)?;
                borsh::BorshSerialize::serialize(&self.last_update, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for LpProvider
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    pool: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    user: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    token_a_provided: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    token_b_provided: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    lp_tokens_owned: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    last_update: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for LpProvider {
            #[inline]
            fn clone(&self) -> LpProvider {
                LpProvider {
                    pool: ::core::clone::Clone::clone(&self.pool),
                    user: ::core::clone::Clone::clone(&self.user),
                    token_a_provided: ::core::clone::Clone::clone(
                        &self.token_a_provided,
                    ),
                    token_b_provided: ::core::clone::Clone::clone(
                        &self.token_b_provided,
                    ),
                    lp_tokens_owned: ::core::clone::Clone::clone(&self.lp_tokens_owned),
                    last_update: ::core::clone::Clone::clone(&self.last_update),
                    bump: ::core::clone::Clone::clone(&self.bump),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for LpProvider {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(LpProvider::DISCRIMINATOR).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for LpProvider {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < LpProvider::DISCRIMINATOR.len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..LpProvider::DISCRIMINATOR.len()];
                if LpProvider::DISCRIMINATOR != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/anchor_project/src/state/lp_provider.rs",
                                        line: 10u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("LpProvider"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[LpProvider::DISCRIMINATOR.len()..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for LpProvider {
            const DISCRIMINATOR: &'static [u8] = &[161, 78, 184, 131, 158, 0, 254, 131];
        }
        #[automatically_derived]
        impl anchor_lang::Owner for LpProvider {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
    }
    pub use lp_provider::*;
}
pub mod events {
    use anchor_lang::prelude::*;
    use crate::TokenMetadata;
    pub struct TokenInitialized {
        pub mint: Pubkey,
        pub name: String,
        pub symbol: String,
        pub creator: Pubkey,
        pub timestamp: i64,
        pub token_metadata_address: Pubkey,
    }
    impl borsh::ser::BorshSerialize for TokenInitialized
    where
        Pubkey: borsh::ser::BorshSerialize,
        String: borsh::ser::BorshSerialize,
        String: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.name, writer)?;
            borsh::BorshSerialize::serialize(&self.symbol, writer)?;
            borsh::BorshSerialize::serialize(&self.creator, writer)?;
            borsh::BorshSerialize::serialize(&self.timestamp, writer)?;
            borsh::BorshSerialize::serialize(&self.token_metadata_address, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for TokenInitialized
    where
        Pubkey: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                name: borsh::BorshDeserialize::deserialize_reader(reader)?,
                symbol: borsh::BorshDeserialize::deserialize_reader(reader)?,
                creator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                timestamp: borsh::BorshDeserialize::deserialize_reader(reader)?,
                token_metadata_address: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
            })
        }
    }
    impl anchor_lang::Event for TokenInitialized {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(TokenInitialized::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    }
    impl anchor_lang::Discriminator for TokenInitialized {
        const DISCRIMINATOR: &'static [u8] = &[77, 70, 233, 124, 236, 92, 204, 0];
    }
    pub struct TokensMinted {
        pub mint: Pubkey,
        pub signer: Pubkey,
        pub to_ata: Pubkey,
    }
    impl borsh::ser::BorshSerialize for TokensMinted
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.signer, writer)?;
            borsh::BorshSerialize::serialize(&self.to_ata, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for TokensMinted
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                signer: borsh::BorshDeserialize::deserialize_reader(reader)?,
                to_ata: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Event for TokensMinted {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(TokensMinted::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    }
    impl anchor_lang::Discriminator for TokensMinted {
        const DISCRIMINATOR: &'static [u8] = &[207, 212, 128, 194, 175, 54, 64, 24];
    }
}
pub mod token_minting {
    pub mod initialize {
        use anchor_lang::prelude::*;
        use anchor_spl::token::{Mint, Token};
        use crate::{events::TokenInitialized, TokenMetadata};
        use crate::TOKEN_METADATA_SEED;
        pub fn _initialize(
            ctx: Context<Initialize>,
            name: String,
            symbol: String,
            uri: String,
            supply: u64,
        ) -> Result<()> {
            let token_metadata = &mut ctx.accounts.token_metadata;
            token_metadata.mint = ctx.accounts.mint.key();
            token_metadata.name = name.clone();
            token_metadata.symbol = symbol.clone();
            token_metadata.uri = uri;
            token_metadata.decimals = 9;
            token_metadata.creator = ctx.accounts.payer.key();
            token_metadata.created_at = Clock::get()?.unix_timestamp;
            token_metadata.supply = supply;
            {
                anchor_lang::solana_program::log::sol_log_data(
                    &[
                        &anchor_lang::Event::data(
                            &TokenInitialized {
                                mint: token_metadata.mint,
                                name,
                                symbol,
                                creator: token_metadata.creator,
                                timestamp: token_metadata.created_at,
                                token_metadata_address: token_metadata.key(),
                            },
                        ),
                    ],
                );
            };
            Ok(())
        }
        pub struct Initialize<'info> {
            #[account(mut)]
            pub payer: Signer<'info>,
            #[account(
                init,
                payer = payer,
                mint::decimals = 9,
                mint::authority = mint_authority,
                mint::freeze_authority = mint_authority,
            )]
            pub mint: Account<'info, Mint>,
            #[account(
                init,
                payer = payer,
                space = 8+TokenMetadata::INIT_SPACE,
                seeds = [TOKEN_METADATA_SEED.as_bytes(),
                payer.key().as_ref(),
                mint.key().as_ref()],
                bump
            )]
            pub token_metadata: Account<'info, TokenMetadata>,
            /// CHECK: PDA only derive and passed, used as authority, no account creation or initialize needed
            #[account(
                seeds = [b"mint_authority",
                payer.key().as_ref(),
                mint.key().as_ref()],
                bump
            )]
            pub mint_authority: UncheckedAccount<'info>,
            pub token_program: Program<'info, Token>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, InitializeBumps> for Initialize<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut InitializeBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("payer"))?;
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let mint = &__accounts[0];
                *__accounts = &__accounts[1..];
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let token_metadata = &__accounts[0];
                *__accounts = &__accounts[1..];
                let mint_authority: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("mint_authority"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let mint: anchor_lang::accounts::account::Account<Mint> = ({
                    #[inline(never)]
                    || {
                        let owner_program = AsRef::<AccountInfo>::as_ref(&mint).owner;
                        if !false
                            || owner_program
                                == &anchor_lang::solana_program::system_program::ID
                        {
                            let __current_lamports = mint.lamports();
                            if __current_lamports == 0 {
                                let space = ::anchor_spl::token::Mint::LEN;
                                let lamports = __anchor_rent.minimum_balance(space);
                                let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                    from: payer.to_account_info(),
                                    to: mint.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::create_account(
                                    cpi_context.with_signer(&[]),
                                    lamports,
                                    space as u64,
                                    &token_program.key(),
                                )?;
                            } else {
                                if payer.key() == mint.key() {
                                    return Err(
                                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .name(),
                                                error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .into(),
                                                error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .to_string(),
                                                error_origin: Some(
                                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/anchor_project/src/token_minting/initialize.rs",
                                                        line: 37u32,
                                                    }),
                                                ),
                                                compared_values: None,
                                            })
                                            .with_pubkeys((payer.key(), mint.key())),
                                    );
                                }
                                let required_lamports = __anchor_rent
                                    .minimum_balance(::anchor_spl::token::Mint::LEN)
                                    .max(1)
                                    .saturating_sub(__current_lamports);
                                if required_lamports > 0 {
                                    let cpi_accounts = anchor_lang::system_program::Transfer {
                                        from: payer.to_account_info(),
                                        to: mint.to_account_info(),
                                    };
                                    let cpi_context = anchor_lang::context::CpiContext::new(
                                        system_program.to_account_info(),
                                        cpi_accounts,
                                    );
                                    anchor_lang::system_program::transfer(
                                        cpi_context,
                                        required_lamports,
                                    )?;
                                }
                                let cpi_accounts = anchor_lang::system_program::Allocate {
                                    account_to_allocate: mint.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::allocate(
                                    cpi_context.with_signer(&[]),
                                    ::anchor_spl::token::Mint::LEN as u64,
                                )?;
                                let cpi_accounts = anchor_lang::system_program::Assign {
                                    account_to_assign: mint.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::assign(
                                    cpi_context.with_signer(&[]),
                                    &token_program.key(),
                                )?;
                            }
                            if let Some(extensions) = Option::<
                                &::anchor_spl::token_interface::ExtensionsVec,
                            >::None {
                                for e in extensions {
                                    match e {
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::GroupPointer => {
                                            ::anchor_spl::token_interface::group_pointer_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::GroupPointerInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::GroupMemberPointer => {
                                            ::anchor_spl::token_interface::group_member_pointer_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::GroupMemberPointerInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::MetadataPointer => {
                                            ::anchor_spl::token_interface::metadata_pointer_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::MetadataPointerInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::MintCloseAuthority => {
                                            ::anchor_spl::token_interface::mint_close_authority_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::MintCloseAuthorityInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<&anchor_lang::prelude::Pubkey>::None,
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::TransferHook => {
                                            ::anchor_spl::token_interface::transfer_hook_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::TransferHookInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                                Option::<anchor_lang::prelude::Pubkey>::None,
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::NonTransferable => {
                                            ::anchor_spl::token_interface::non_transferable_mint_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::NonTransferableMintInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: mint.to_account_info(),
                                                    },
                                                ),
                                            )?;
                                        }
                                        ::anchor_spl::token_interface::spl_token_2022::extension::ExtensionType::PermanentDelegate => {
                                            ::anchor_spl::token_interface::permanent_delegate_initialize(
                                                anchor_lang::context::CpiContext::new(
                                                    token_program.to_account_info(),
                                                    ::anchor_spl::token_interface::PermanentDelegateInitialize {
                                                        token_program_id: token_program.to_account_info(),
                                                        mint: mint.to_account_info(),
                                                    },
                                                ),
                                                Option::<&anchor_lang::prelude::Pubkey>::None.unwrap(),
                                            )?;
                                        }
                                        _ => {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "not implemented: {0}",
                                                    format_args!("{0:?}", e),
                                                ),
                                            );
                                        }
                                    }
                                }
                            }
                            let cpi_program = token_program.to_account_info();
                            let accounts = ::anchor_spl::token_interface::InitializeMint2 {
                                mint: mint.to_account_info(),
                            };
                            let cpi_ctx = anchor_lang::context::CpiContext::new(
                                cpi_program,
                                accounts,
                            );
                            ::anchor_spl::token_interface::initialize_mint2(
                                cpi_ctx,
                                9,
                                &mint_authority.key(),
                                Option::<
                                    &anchor_lang::prelude::Pubkey,
                                >::Some(&mint_authority.key()),
                            )?;
                        }
                        let pa: anchor_lang::accounts::account::Account<Mint> = match anchor_lang::accounts::account::Account::try_from_unchecked(
                            &mint,
                        ) {
                            Ok(val) => val,
                            Err(e) => return Err(e.with_account_name("mint")),
                        };
                        if false {
                            if pa.mint_authority
                                != anchor_lang::solana_program::program_option::COption::Some(
                                    mint_authority.key(),
                                )
                            {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintMintMintAuthority,
                                        )
                                        .with_account_name("mint"),
                                );
                            }
                            if pa
                                .freeze_authority
                                .as_ref()
                                .map(|fa| {
                                    Option::<
                                        &anchor_lang::prelude::Pubkey,
                                    >::Some(&mint_authority.key())
                                        .as_ref()
                                        .map(|expected_fa| fa != *expected_fa)
                                        .unwrap_or(true)
                                })
                                .unwrap_or(
                                    Option::<
                                        &anchor_lang::prelude::Pubkey,
                                    >::Some(&mint_authority.key())
                                        .is_some(),
                                )
                            {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintMintFreezeAuthority,
                                        )
                                        .with_account_name("mint"),
                                );
                            }
                            if pa.decimals != 9 {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintMintDecimals,
                                        )
                                        .with_account_name("mint")
                                        .with_values((pa.decimals, 9)),
                                );
                            }
                            if owner_program != &token_program.key() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintMintTokenProgram,
                                        )
                                        .with_account_name("mint")
                                        .with_pubkeys((*owner_program, token_program.key())),
                                );
                            }
                        }
                        Ok(pa)
                    }
                })()?;
                if !AsRef::<AccountInfo>::as_ref(&mint).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("mint"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&mint).is_signer {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSigner,
                            )
                            .with_account_name("mint"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        mint.to_account_info().lamports(),
                        mint.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("mint"),
                    );
                }
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        TOKEN_METADATA_SEED.as_bytes(),
                        payer.key().as_ref(),
                        mint.key().as_ref(),
                    ],
                    __program_id,
                );
                __bumps.token_metadata = __bump;
                if token_metadata.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("token_metadata")
                            .with_pubkeys((token_metadata.key(), __pda_address)),
                    );
                }
                let token_metadata = ({
                    #[inline(never)]
                    || {
                        let actual_field = AsRef::<AccountInfo>::as_ref(&token_metadata);
                        let actual_owner = actual_field.owner;
                        let space = 8 + TokenMetadata::INIT_SPACE;
                        let pa: anchor_lang::accounts::account::Account<TokenMetadata> = if !false
                            || actual_owner
                                == &anchor_lang::solana_program::system_program::ID
                        {
                            let __current_lamports = token_metadata.lamports();
                            if __current_lamports == 0 {
                                let space = space;
                                let lamports = __anchor_rent.minimum_balance(space);
                                let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                    from: payer.to_account_info(),
                                    to: token_metadata.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::create_account(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    TOKEN_METADATA_SEED.as_bytes(),
                                                    payer.key().as_ref(),
                                                    mint.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    lamports,
                                    space as u64,
                                    __program_id,
                                )?;
                            } else {
                                if payer.key() == token_metadata.key() {
                                    return Err(
                                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                                error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .name(),
                                                error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .into(),
                                                error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                                    .to_string(),
                                                error_origin: Some(
                                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                                        filename: "programs/anchor_project/src/token_minting/initialize.rs",
                                                        line: 37u32,
                                                    }),
                                                ),
                                                compared_values: None,
                                            })
                                            .with_pubkeys((payer.key(), token_metadata.key())),
                                    );
                                }
                                let required_lamports = __anchor_rent
                                    .minimum_balance(space)
                                    .max(1)
                                    .saturating_sub(__current_lamports);
                                if required_lamports > 0 {
                                    let cpi_accounts = anchor_lang::system_program::Transfer {
                                        from: payer.to_account_info(),
                                        to: token_metadata.to_account_info(),
                                    };
                                    let cpi_context = anchor_lang::context::CpiContext::new(
                                        system_program.to_account_info(),
                                        cpi_accounts,
                                    );
                                    anchor_lang::system_program::transfer(
                                        cpi_context,
                                        required_lamports,
                                    )?;
                                }
                                let cpi_accounts = anchor_lang::system_program::Allocate {
                                    account_to_allocate: token_metadata.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::allocate(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    TOKEN_METADATA_SEED.as_bytes(),
                                                    payer.key().as_ref(),
                                                    mint.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    space as u64,
                                )?;
                                let cpi_accounts = anchor_lang::system_program::Assign {
                                    account_to_assign: token_metadata.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::assign(
                                    cpi_context
                                        .with_signer(
                                            &[
                                                &[
                                                    TOKEN_METADATA_SEED.as_bytes(),
                                                    payer.key().as_ref(),
                                                    mint.key().as_ref(),
                                                    &[__bump][..],
                                                ][..],
                                            ],
                                        ),
                                    __program_id,
                                )?;
                            }
                            match anchor_lang::accounts::account::Account::try_from_unchecked(
                                &token_metadata,
                            ) {
                                Ok(val) => val,
                                Err(e) => return Err(e.with_account_name("token_metadata")),
                            }
                        } else {
                            match anchor_lang::accounts::account::Account::try_from(
                                &token_metadata,
                            ) {
                                Ok(val) => val,
                                Err(e) => return Err(e.with_account_name("token_metadata")),
                            }
                        };
                        if false {
                            if space != actual_field.data_len() {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintSpace,
                                        )
                                        .with_account_name("token_metadata")
                                        .with_values((space, actual_field.data_len())),
                                );
                            }
                            if actual_owner != __program_id {
                                return Err(
                                    anchor_lang::error::Error::from(
                                            anchor_lang::error::ErrorCode::ConstraintOwner,
                                        )
                                        .with_account_name("token_metadata")
                                        .with_pubkeys((*actual_owner, *__program_id)),
                                );
                            }
                            {
                                let required_lamports = __anchor_rent
                                    .minimum_balance(space);
                                if pa.to_account_info().lamports() < required_lamports {
                                    return Err(
                                        anchor_lang::error::Error::from(
                                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                            )
                                            .with_account_name("token_metadata"),
                                    );
                                }
                            }
                        }
                        Ok(pa)
                    }
                })()?;
                if !AsRef::<AccountInfo>::as_ref(&token_metadata).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("token_metadata"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        token_metadata.to_account_info().lamports(),
                        token_metadata.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("token_metadata"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("payer"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"mint_authority", payer.key().as_ref(), mint.key().as_ref()],
                    &__program_id,
                );
                __bumps.mint_authority = __bump;
                if mint_authority.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("mint_authority")
                            .with_pubkeys((mint_authority.key(), __pda_address)),
                    );
                }
                Ok(Initialize {
                    payer,
                    mint,
                    token_metadata,
                    mint_authority,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.token_metadata.to_account_infos());
                account_infos.extend(self.mint_authority.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Initialize<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(Some(true)));
                account_metas.extend(self.token_metadata.to_account_metas(None));
                account_metas.extend(self.mint_authority.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Initialize<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                anchor_lang::AccountsExit::exit(&self.mint, program_id)
                    .map_err(|e| e.with_account_name("mint"))?;
                anchor_lang::AccountsExit::exit(&self.token_metadata, program_id)
                    .map_err(|e| e.with_account_name("token_metadata"))?;
                Ok(())
            }
        }
        pub struct InitializeBumps {
            pub token_metadata: u8,
            pub mint_authority: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InitializeBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "InitializeBumps",
                    "token_metadata",
                    &self.token_metadata,
                    "mint_authority",
                    &&self.mint_authority,
                )
            }
        }
        impl Default for InitializeBumps {
            fn default() -> Self {
                InitializeBumps {
                    token_metadata: u8::MAX,
                    mint_authority: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for Initialize<'info>
        where
            'info: 'info,
        {
            type Bumps = InitializeBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_initialize {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`Initialize`].
            pub struct Initialize {
                pub payer: Pubkey,
                pub mint: Pubkey,
                pub token_metadata: Pubkey,
                pub mint_authority: Pubkey,
                pub token_program: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for Initialize
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_metadata, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint_authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Initialize {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.payer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.mint,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.token_metadata,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.mint_authority,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_initialize {
            use super::*;
            /// Generated CPI struct of the accounts for [`Initialize`].
            pub struct Initialize<'info> {
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_metadata: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub mint_authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Initialize<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.payer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.mint),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.token_metadata),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.mint_authority),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.payer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.mint),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_metadata,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.mint_authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
    }
    pub use initialize::*;
    pub mod state {
        use anchor_lang::prelude::*;
        pub struct TokenMetadata {
            pub mint: Pubkey,
            #[max_len(20)]
            pub name: String,
            #[max_len(3)]
            pub symbol: String,
            pub supply: u64,
            pub decimals: u8,
            #[max_len(255)]
            pub uri: String,
            pub creator: Pubkey,
            pub created_at: i64,
        }
        #[automatically_derived]
        impl anchor_lang::Space for TokenMetadata {
            const INIT_SPACE: usize = 0 + 32 + (4 + 20) + (4 + 3) + 8 + 1 + (4 + 255)
                + 32 + 8;
        }
        impl borsh::ser::BorshSerialize for TokenMetadata
        where
            Pubkey: borsh::ser::BorshSerialize,
            String: borsh::ser::BorshSerialize,
            String: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            String: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.mint, writer)?;
                borsh::BorshSerialize::serialize(&self.name, writer)?;
                borsh::BorshSerialize::serialize(&self.symbol, writer)?;
                borsh::BorshSerialize::serialize(&self.supply, writer)?;
                borsh::BorshSerialize::serialize(&self.decimals, writer)?;
                borsh::BorshSerialize::serialize(&self.uri, writer)?;
                borsh::BorshSerialize::serialize(&self.creator, writer)?;
                borsh::BorshSerialize::serialize(&self.created_at, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for TokenMetadata
        where
            Pubkey: borsh::BorshDeserialize,
            String: borsh::BorshDeserialize,
            String: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            String: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    name: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    symbol: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    supply: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    decimals: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    uri: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    creator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    created_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TokenMetadata {
            #[inline]
            fn clone(&self) -> TokenMetadata {
                TokenMetadata {
                    mint: ::core::clone::Clone::clone(&self.mint),
                    name: ::core::clone::Clone::clone(&self.name),
                    symbol: ::core::clone::Clone::clone(&self.symbol),
                    supply: ::core::clone::Clone::clone(&self.supply),
                    decimals: ::core::clone::Clone::clone(&self.decimals),
                    uri: ::core::clone::Clone::clone(&self.uri),
                    creator: ::core::clone::Clone::clone(&self.creator),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for TokenMetadata {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(TokenMetadata::DISCRIMINATOR).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for TokenMetadata {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < TokenMetadata::DISCRIMINATOR.len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..TokenMetadata::DISCRIMINATOR.len()];
                if TokenMetadata::DISCRIMINATOR != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/anchor_project/src/token_minting/state.rs",
                                        line: 3u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("TokenMetadata"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[TokenMetadata::DISCRIMINATOR.len()..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for TokenMetadata {
            const DISCRIMINATOR: &'static [u8] = &[
                237, 215, 132, 182, 24, 127, 175, 173,
            ];
        }
        #[automatically_derived]
        impl anchor_lang::Owner for TokenMetadata {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
    }
    pub use state::*;
    pub mod mint {
        use anchor_lang::prelude::*;
        use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};
        use crate::events::TokensMinted;
        pub fn _mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
            let mint_account = ctx.accounts.mint.key();
            let seeds = &[
                b"mint_authority",
                ctx.accounts.signer.key.as_ref(),
                mint_account.as_ref(),
                &[ctx.bumps.mint_authority],
            ];
            let signer_seeds = &[&seeds[..]];
            let cpi_ctx = ctx.accounts.into_mint_to_context().with_signer(signer_seeds);
            mint_to(cpi_ctx, amount)?;
            {
                anchor_lang::solana_program::log::sol_log_data(
                    &[
                        &anchor_lang::Event::data(
                            &TokensMinted {
                                mint: mint_account,
                                signer: ctx.accounts.signer.key(),
                                to_ata: ctx.accounts.to.key(),
                            },
                        ),
                    ],
                );
            };
            Ok(())
        }
        pub struct MintTokens<'info> {
            pub signer: Signer<'info>,
            #[account(mut)]
            pub mint: Account<'info, Mint>,
            #[account(mut)]
            pub to: Account<'info, TokenAccount>,
            /// CHECK: This is the mint authority and must match the mint's authority
            #[account(
                seeds = [b"mint_authority",
                signer.key().as_ref(),
                mint.key().as_ref()],
                bump
            )]
            pub mint_authority: UncheckedAccount<'info>,
            pub token_program: Program<'info, Token>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, MintTokensBumps> for MintTokens<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut MintTokensBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let signer: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("signer"))?;
                let mint: anchor_lang::accounts::account::Account<Mint> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("mint"))?;
                let to: anchor_lang::accounts::account::Account<TokenAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("to"))?;
                let mint_authority: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("mint_authority"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                if !AsRef::<AccountInfo>::as_ref(&mint).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("mint"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&to).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("to"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"mint_authority", signer.key().as_ref(), mint.key().as_ref()],
                    &__program_id,
                );
                __bumps.mint_authority = __bump;
                if mint_authority.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("mint_authority")
                            .with_pubkeys((mint_authority.key(), __pda_address)),
                    );
                }
                Ok(MintTokens {
                    signer,
                    mint,
                    to,
                    mint_authority,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for MintTokens<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.signer.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.to.to_account_infos());
                account_infos.extend(self.mint_authority.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for MintTokens<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.signer.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.to.to_account_metas(None));
                account_metas.extend(self.mint_authority.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for MintTokens<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.mint, program_id)
                    .map_err(|e| e.with_account_name("mint"))?;
                anchor_lang::AccountsExit::exit(&self.to, program_id)
                    .map_err(|e| e.with_account_name("to"))?;
                Ok(())
            }
        }
        pub struct MintTokensBumps {
            pub mint_authority: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MintTokensBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "MintTokensBumps",
                    "mint_authority",
                    &&self.mint_authority,
                )
            }
        }
        impl Default for MintTokensBumps {
            fn default() -> Self {
                MintTokensBumps {
                    mint_authority: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for MintTokens<'info>
        where
            'info: 'info,
        {
            type Bumps = MintTokensBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_mint_tokens {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`MintTokens`].
            pub struct MintTokens {
                pub signer: Pubkey,
                pub mint: Pubkey,
                pub to: Pubkey,
                pub mint_authority: Pubkey,
                pub token_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for MintTokens
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.to, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint_authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for MintTokens {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.signer,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.mint,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.to,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.mint_authority,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.token_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_mint_tokens {
            use super::*;
            /// Generated CPI struct of the accounts for [`MintTokens`].
            pub struct MintTokens<'info> {
                pub signer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub to: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub mint_authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for MintTokens<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.signer),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.mint),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.to),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.mint_authority),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.token_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for MintTokens<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.signer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.mint),
                        );
                    account_infos
                        .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.to));
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.mint_authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.token_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        impl<'info> MintTokens<'info> {
            pub fn into_mint_to_context(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
                CpiContext::new(
                    self.token_program.to_account_info(),
                    MintTo {
                        mint: self.mint.to_account_info(),
                        to: self.to.to_account_info(),
                        authority: self.mint_authority.to_account_info(),
                    },
                )
            }
        }
    }
    pub use mint::*;
}
pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;
pub use token_minting::*;
use self::anchor_project::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) = unsafe {
        ::solana_program_entrypoint::deserialize(input)
    };
    match entry(program_id, &accounts, instruction_data) {
        Ok(()) => ::solana_program_entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one category for now.
///
/// Global methods - regular methods inside of the `#[program]`.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Check whether the declared program id matches the input program
///   id. If it's not, return an error.
/// * Find and invoke the method based on whether the instruction data
///   starts with the method's discriminator.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data)
        .map_err(|e| {
            e.log();
            e.into()
        })
}
fn try_entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct AnchorProject;
    #[automatically_derived]
    impl ::core::clone::Clone for AnchorProject {
        #[inline]
        fn clone(&self) -> AnchorProject {
            AnchorProject
        }
    }
    impl anchor_lang::Id for AnchorProject {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each instruction's discriminator is checked until the given instruction data starts with
/// the current discriminator.
///
/// If a match is found, the instruction handler is called using the given instruction data
/// excluding the prepended discriminator bytes.
///
/// If no match is found, the fallback function is executed if it exists, or an error is
/// returned if it doesn't exist.
fn dispatch<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if data.starts_with(instruction::InitializeLiquidityPool::DISCRIMINATOR) {
        return __private::__global::initialize_liquidity_pool(
            program_id,
            accounts,
            &data[instruction::InitializeLiquidityPool::DISCRIMINATOR.len()..],
        );
    }
    if data.starts_with(instruction::AddLiquidity::DISCRIMINATOR) {
        return __private::__global::add_liquidity(
            program_id,
            accounts,
            &data[instruction::AddLiquidity::DISCRIMINATOR.len()..],
        );
    }
    if data.starts_with(instruction::RemoveLiquidity::DISCRIMINATOR) {
        return __private::__global::remove_liquidity(
            program_id,
            accounts,
            &data[instruction::RemoveLiquidity::DISCRIMINATOR.len()..],
        );
    }
    if data.starts_with(instruction::Swap::DISCRIMINATOR) {
        return __private::__global::swap(
            program_id,
            accounts,
            &data[instruction::Swap::DISCRIMINATOR.len()..],
        );
    }
    if data.starts_with(instruction::CollectFees::DISCRIMINATOR) {
        return __private::__global::collect_fees(
            program_id,
            accounts,
            &data[instruction::CollectFees::DISCRIMINATOR.len()..],
        );
    }
    if data.starts_with(instruction::InitializeMintAccount::DISCRIMINATOR) {
        return __private::__global::initialize_mint_account(
            program_id,
            accounts,
            &data[instruction::InitializeMintAccount::DISCRIMINATOR.len()..],
        );
    }
    if data.starts_with(instruction::MintTokens::DISCRIMINATOR) {
        return __private::__global::mint_tokens(
            program_id,
            accounts,
            &data[instruction::MintTokens::DISCRIMINATOR.len()..],
        );
    }
    if data.starts_with(anchor_lang::idl::IDL_IX_TAG_LE) {
        return __private::__idl::__idl_dispatch(
            program_id,
            accounts,
            &data[anchor_lang::idl::IDL_IX_TAG_LE.len()..],
        );
    }
    if data.starts_with(anchor_lang::event::EVENT_IX_TAG_LE) {
        return Err(anchor_lang::error::ErrorCode::EventInstructionStub.into());
    }
    Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into())
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        pub fn __idl_dispatch<'info>(
            program_id: &Pubkey,
            accounts: &'info [AccountInfo<'info>],
            idl_ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = <IdlCreateAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Resize { data_len } => {
                    let mut bumps = <IdlResizeAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlResizeAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_resize_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Close => {
                    let mut bumps = <IdlCloseAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCloseAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_close_account(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = <IdlCreateBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = <IdlSetBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        use anchor_lang::idl::ERASED_AUTHORITY;
        pub struct IdlAccount {
            pub authority: Pubkey,
            pub data_len: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccount {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "IdlAccount",
                    "authority",
                    &self.authority,
                    "data_len",
                    &&self.data_len,
                )
            }
        }
        impl borsh::ser::BorshSerialize for IdlAccount
        where
            Pubkey: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.data_len, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for IdlAccount
        where
            Pubkey: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    data_len: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IdlAccount {
            #[inline]
            fn clone(&self) -> IdlAccount {
                IdlAccount {
                    authority: ::core::clone::Clone::clone(&self.authority),
                    data_len: ::core::clone::Clone::clone(&self.data_len),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for IdlAccount {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(IdlAccount::DISCRIMINATOR).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for IdlAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < IdlAccount::DISCRIMINATOR.len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..IdlAccount::DISCRIMINATOR.len()];
                if IdlAccount::DISCRIMINATOR != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/anchor_project/src/lib.rs",
                                        line: 19u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("IdlAccount"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[IdlAccount::DISCRIMINATOR.len()..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for IdlAccount {
            const DISCRIMINATOR: &'static [u8] = &[24, 70, 98, 191, 58, 144, 123, 158];
        }
        impl IdlAccount {
            pub fn address(program_id: &Pubkey) -> Pubkey {
                let program_signer = Pubkey::find_program_address(&[], program_id).0;
                Pubkey::create_with_seed(&program_signer, IdlAccount::seed(), program_id)
                    .expect("Seed is always valid")
            }
            pub fn seed() -> &'static str {
                "anchor:idl"
            }
        }
        impl anchor_lang::Owner for IdlAccount {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        pub struct IdlCreateAccounts<'info> {
            #[account(signer)]
            pub from: AccountInfo<'info>,
            #[account(mut)]
            pub to: AccountInfo<'info>,
            #[account(seeds = [], bump)]
            pub base: AccountInfo<'info>,
            pub system_program: Program<'info, System>,
            #[account(executable)]
            pub program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateAccountsBumps>
        for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCreateAccountsBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let from: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("from"))?;
                let to: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("to"))?;
                let base: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("base"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("program"))?;
                if !&from.is_signer {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSigner,
                            )
                            .with_account_name("from"),
                    );
                }
                if !&to.is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("to"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[],
                    &__program_id,
                );
                __bumps.base = __bump;
                if base.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("base")
                            .with_pubkeys((base.key(), __pda_address)),
                    );
                }
                if !&program.executable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintExecutable,
                            )
                            .with_account_name("program"),
                    );
                }
                Ok(IdlCreateAccounts {
                    from,
                    to,
                    base,
                    system_program,
                    program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.from.to_account_infos());
                account_infos.extend(self.to.to_account_infos());
                account_infos.extend(self.base.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.from.to_account_metas(Some(true)));
                account_metas.extend(self.to.to_account_metas(None));
                account_metas.extend(self.base.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.to, program_id)
                    .map_err(|e| e.with_account_name("to"))?;
                Ok(())
            }
        }
        pub struct IdlCreateAccountsBumps {
            pub base: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "IdlCreateAccountsBumps",
                    "base",
                    &&self.base,
                )
            }
        }
        impl Default for IdlCreateAccountsBumps {
            fn default() -> Self {
                IdlCreateAccountsBumps {
                    base: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCreateAccountsBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_create_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts {
                pub from: Pubkey,
                pub to: Pubkey,
                pub base: Pubkey,
                pub system_program: Pubkey,
                pub program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateAccounts
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.from, writer)?;
                    borsh::BorshSerialize::serialize(&self.to, writer)?;
                    borsh::BorshSerialize::serialize(&self.base, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.from,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.to,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.base,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_create_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts<'info> {
                pub from: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub to: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub base: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.from),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.to),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.base),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.from),
                        );
                    account_infos
                        .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.to));
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.base),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlAccounts<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlAccountsBumps> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlAccountsBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlAccounts { idl, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlAccountsBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlAccountsBumps")
            }
        }
        impl Default for IdlAccountsBumps {
            fn default() -> Self {
                IdlAccountsBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlAccounts<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlAccountsBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlAccounts`].
            pub struct IdlAccounts {
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlAccounts
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlAccounts`].
            pub struct IdlAccounts<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlResizeAccount<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(mut, constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlResizeAccountBumps>
        for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlResizeAccountBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&authority).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlResizeAccount {
                    idl,
                    authority,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                anchor_lang::AccountsExit::exit(&self.authority, program_id)
                    .map_err(|e| e.with_account_name("authority"))?;
                Ok(())
            }
        }
        pub struct IdlResizeAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlResizeAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlResizeAccountBumps")
            }
        }
        impl Default for IdlResizeAccountBumps {
            fn default() -> Self {
                IdlResizeAccountBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlResizeAccountBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_resize_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount {
                pub idl: Pubkey,
                pub authority: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlResizeAccount
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlResizeAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_resize_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCreateBuffer<'info> {
            #[account(zero)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateBufferBumps>
        for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCreateBufferBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let buffer = &__accounts[0];
                *__accounts = &__accounts[1..];
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let __anchor_rent = Rent::get()?;
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = {
                    let mut __data: &[u8] = &buffer.try_borrow_data()?;
                    let __disc = &__data[..IdlAccount::DISCRIMINATOR.len()];
                    let __has_disc = __disc.iter().any(|b| *b != 0);
                    if __has_disc {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintZero,
                                )
                                .with_account_name("buffer"),
                        );
                    }
                    match anchor_lang::accounts::account::Account::try_from_unchecked(
                        &buffer,
                    ) {
                        Ok(val) => val,
                        Err(e) => return Err(e.with_account_name("buffer")),
                    }
                };
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        buffer.to_account_info().lamports(),
                        buffer.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlCreateBuffer {
                    buffer,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
                Ok(())
            }
        }
        pub struct IdlCreateBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCreateBufferBumps")
            }
        }
        impl Default for IdlCreateBufferBumps {
            fn default() -> Self {
                IdlCreateBufferBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCreateBufferBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_create_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer {
                pub buffer: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateBuffer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_create_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlSetBuffer<'info> {
            #[account(mut, constraint = buffer.authority = = idl.authority)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlSetBufferBumps>
        for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlSetBufferBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("buffer"))?;
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(buffer.authority == idl.authority) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlSetBuffer {
                    buffer,
                    idl,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlSetBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlSetBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlSetBufferBumps")
            }
        }
        impl Default for IdlSetBufferBumps {
            fn default() -> Self {
                IdlSetBufferBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlSetBufferBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_set_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer {
                pub buffer: Pubkey,
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlSetBuffer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlSetBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_set_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCloseAccount<'info> {
            #[account(mut, has_one = authority, close = sol_destination)]
            pub account: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            #[account(mut)]
            pub sol_destination: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCloseAccountBumps>
        for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCloseAccountBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let account: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("account"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let sol_destination: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("sol_destination"))?;
                if !AsRef::<AccountInfo>::as_ref(&account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("account"),
                    );
                }
                {
                    let my_key = account.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("account")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                {
                    if account.key() == sol_destination.key() {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintClose,
                                )
                                .with_account_name("account"),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !&sol_destination.is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("sol_destination"),
                    );
                }
                Ok(IdlCloseAccount {
                    account,
                    authority,
                    sol_destination,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.account.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.sol_destination.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.account.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.sol_destination.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                {
                    let sol_destination = &self.sol_destination;
                    anchor_lang::AccountsClose::close(
                            &self.account,
                            sol_destination.to_account_info(),
                        )
                        .map_err(|e| e.with_account_name("account"))?;
                }
                anchor_lang::AccountsExit::exit(&self.sol_destination, program_id)
                    .map_err(|e| e.with_account_name("sol_destination"))?;
                Ok(())
            }
        }
        pub struct IdlCloseAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCloseAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCloseAccountBumps")
            }
        }
        impl Default for IdlCloseAccountBumps {
            fn default() -> Self {
                IdlCloseAccountBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCloseAccountBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_close_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount {
                pub account: Pubkey,
                pub authority: Pubkey,
                pub sol_destination: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCloseAccount
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.account, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.sol_destination, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCloseAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.sol_destination,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_close_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount<'info> {
                pub account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub sol_destination: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.sol_destination),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.account),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.sol_destination,
                            ),
                        );
                    account_infos
                }
            }
        }
        use std::cell::{Ref, RefMut};
        pub trait IdlTrailingData<'info> {
            fn trailing_data(self) -> Ref<'info, [u8]>;
            fn trailing_data_mut(self) -> RefMut<'info, [u8]>;
        }
        impl<'a, 'info: 'a> IdlTrailingData<'a> for &'a Account<'info, IdlAccount> {
            fn trailing_data(self) -> Ref<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                Ref::map(info.try_borrow_data().unwrap(), |d| &d[44..])
            }
            fn trailing_data_mut(self) -> RefMut<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                RefMut::map(info.try_borrow_mut_data().unwrap(), |d| &mut d[44..])
            }
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(
                    anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into(),
                );
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = std::cmp::min(
                IdlAccount::DISCRIMINATOR.len() + 32 + 4 + data_len as usize,
                10_000,
            );
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_resize_account(
            program_id: &Pubkey,
            accounts: &mut IdlResizeAccount,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: IdlResizeAccount");
            let data_len: usize = data_len as usize;
            if accounts.idl.data_len != 0 {
                return Err(anchor_lang::error::ErrorCode::IdlAccountNotEmpty.into());
            }
            let idl_ref = AsRef::<AccountInfo>::as_ref(&accounts.idl);
            let new_account_space = idl_ref
                .data_len()
                .checked_add(
                    std::cmp::min(
                        data_len
                            .checked_sub(idl_ref.data_len())
                            .expect(
                                "data_len should always be >= the current account space",
                            ),
                        10_000,
                    ),
                )
                .unwrap();
            if new_account_space > idl_ref.data_len() {
                let sysvar_rent = Rent::get()?;
                let new_rent_minimum = sysvar_rent.minimum_balance(new_account_space);
                anchor_lang::system_program::transfer(
                    anchor_lang::context::CpiContext::new(
                        accounts.system_program.to_account_info(),
                        anchor_lang::system_program::Transfer {
                            from: accounts.authority.to_account_info(),
                            to: accounts.idl.to_account_info(),
                        },
                    ),
                    new_rent_minimum.checked_sub(idl_ref.lamports()).unwrap(),
                )?;
                idl_ref.realloc(new_account_space, false)?;
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_close_account(
            program_id: &Pubkey,
            accounts: &mut IdlCloseAccount,
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: IdlCloseAccount");
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: IdlWrite");
            let prev_len: usize = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.idl.data_len)
                .unwrap();
            let new_len: usize = prev_len.checked_add(idl_data.len()).unwrap() as usize;
            accounts.idl.data_len = accounts
                .idl
                .data_len
                .checked_add(
                    ::std::convert::TryInto::<u32>::try_into(idl_data.len()).unwrap(),
                )
                .unwrap();
            use IdlTrailingData;
            let mut idl_bytes = accounts.idl.trailing_data_mut();
            let idl_expansion = &mut idl_bytes[prev_len..new_len];
            if idl_expansion.len() != idl_data.len() {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireEqViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireEqViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireEqViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/anchor_project/src/lib.rs",
                                    line: 19u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((idl_expansion.len(), idl_data.len())),
                );
            }
            idl_expansion.copy_from_slice(&idl_data[..]);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data_len = accounts.buffer.data_len;
            use IdlTrailingData;
            let buffer_len = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.buffer.data_len)
                .unwrap();
            let mut target = accounts.idl.trailing_data_mut();
            let source = &accounts.buffer.trailing_data()[..buffer_len];
            if target.len() < buffer_len {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireGteViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireGteViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireGteViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/anchor_project/src/lib.rs",
                                    line: 19u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((target.len(), buffer_len)),
                );
            }
            target[..buffer_len].copy_from_slice(source);
            Ok(())
        }
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn initialize_liquidity_pool<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: InitializeLiquidityPool");
            let ix = instruction::InitializeLiquidityPool::deserialize(
                    &mut &__ix_data[..],
                )
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::InitializeLiquidityPool = ix;
            let mut __bumps = <InitializeLiquidityPool as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = InitializeLiquidityPool::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = anchor_project::initialize_liquidity_pool(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn add_liquidity<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: AddLiquidity");
            let ix = instruction::AddLiquidity::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::AddLiquidity { amount_a, amount_b } = ix;
            let mut __bumps = <AddLiquidity as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = AddLiquidity::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = anchor_project::add_liquidity(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                amount_a,
                amount_b,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn remove_liquidity<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: RemoveLiquidity");
            let ix = instruction::RemoveLiquidity::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::RemoveLiquidity = ix;
            let mut __bumps = <RemoveLiquidity as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = RemoveLiquidity::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = anchor_project::remove_liquidity(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn swap<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: Swap");
            let ix = instruction::Swap::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Swap = ix;
            let mut __bumps = <SwapTokens as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = SwapTokens::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = anchor_project::swap(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn collect_fees<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: CollectFees");
            let ix = instruction::CollectFees::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::CollectFees = ix;
            let mut __bumps = <CollectFees as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = CollectFees::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = anchor_project::collect_fees(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn initialize_mint_account<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: InitializeMintAccount");
            let ix = instruction::InitializeMintAccount::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::InitializeMintAccount { name, symbol, uri, supply } = ix;
            let mut __bumps = <Initialize as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Initialize::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = anchor_project::initialize_mint_account(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                name,
                symbol,
                uri,
                supply,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn mint_tokens<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_msg::sol_log("Instruction: MintTokens");
            let ix = instruction::MintTokens::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::MintTokens { amount } = ix;
            let mut __bumps = <MintTokens as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = MintTokens::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = anchor_project::mint_tokens(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                amount,
            )?;
            __accounts.exit(__program_id)
        }
    }
}
pub mod anchor_project {
    use super::*;
    pub fn initialize_liquidity_pool(
        ctx: Context<InitializeLiquidityPool>,
    ) -> Result<()> {
        _initialize_liquidity_pool(ctx)
    }
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        _add_liquidity(ctx, amount_a, amount_b)
    }
    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>) -> Result<()> {
        _remove_liquidity(ctx)
    }
    pub fn swap(ctx: Context<SwapTokens>) -> Result<()> {
        _swap(ctx)
    }
    pub fn collect_fees(ctx: Context<CollectFees>) -> Result<()> {
        _collect_fees(ctx)
    }
    pub fn initialize_mint_account(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        uri: String,
        supply: u64,
    ) -> Result<()> {
        _initialize(ctx, name, symbol, uri, supply)
    }
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        _mint_tokens(ctx, amount)
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction.
    pub struct InitializeLiquidityPool;
    impl borsh::ser::BorshSerialize for InitializeLiquidityPool {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeLiquidityPool {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for InitializeLiquidityPool {
        const DISCRIMINATOR: &'static [u8] = &[155, 18, 138, 107, 111, 23, 178, 178];
    }
    impl anchor_lang::InstructionData for InitializeLiquidityPool {}
    impl anchor_lang::Owner for InitializeLiquidityPool {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct AddLiquidity {
        pub amount_a: u64,
        pub amount_b: u64,
    }
    impl borsh::ser::BorshSerialize for AddLiquidity
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount_a, writer)?;
            borsh::BorshSerialize::serialize(&self.amount_b, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AddLiquidity
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount_a: borsh::BorshDeserialize::deserialize_reader(reader)?,
                amount_b: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for AddLiquidity {
        const DISCRIMINATOR: &'static [u8] = &[181, 157, 89, 67, 143, 182, 52, 72];
    }
    impl anchor_lang::InstructionData for AddLiquidity {}
    impl anchor_lang::Owner for AddLiquidity {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct RemoveLiquidity;
    impl borsh::ser::BorshSerialize for RemoveLiquidity {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for RemoveLiquidity {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for RemoveLiquidity {
        const DISCRIMINATOR: &'static [u8] = &[80, 85, 209, 72, 24, 206, 177, 108];
    }
    impl anchor_lang::InstructionData for RemoveLiquidity {}
    impl anchor_lang::Owner for RemoveLiquidity {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct Swap;
    impl borsh::ser::BorshSerialize for Swap {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Swap {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for Swap {
        const DISCRIMINATOR: &'static [u8] = &[248, 198, 158, 145, 225, 117, 135, 200];
    }
    impl anchor_lang::InstructionData for Swap {}
    impl anchor_lang::Owner for Swap {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct CollectFees;
    impl borsh::ser::BorshSerialize for CollectFees {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CollectFees {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for CollectFees {
        const DISCRIMINATOR: &'static [u8] = &[164, 152, 207, 99, 30, 186, 19, 182];
    }
    impl anchor_lang::InstructionData for CollectFees {}
    impl anchor_lang::Owner for CollectFees {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct InitializeMintAccount {
        pub name: String,
        pub symbol: String,
        pub uri: String,
        pub supply: u64,
    }
    impl borsh::ser::BorshSerialize for InitializeMintAccount
    where
        String: borsh::ser::BorshSerialize,
        String: borsh::ser::BorshSerialize,
        String: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.name, writer)?;
            borsh::BorshSerialize::serialize(&self.symbol, writer)?;
            borsh::BorshSerialize::serialize(&self.uri, writer)?;
            borsh::BorshSerialize::serialize(&self.supply, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeMintAccount
    where
        String: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                name: borsh::BorshDeserialize::deserialize_reader(reader)?,
                symbol: borsh::BorshDeserialize::deserialize_reader(reader)?,
                uri: borsh::BorshDeserialize::deserialize_reader(reader)?,
                supply: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for InitializeMintAccount {
        const DISCRIMINATOR: &'static [u8] = &[151, 211, 156, 193, 234, 128, 159, 249];
    }
    impl anchor_lang::InstructionData for InitializeMintAccount {}
    impl anchor_lang::Owner for InitializeMintAccount {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct MintTokens {
        pub amount: u64,
    }
    impl borsh::ser::BorshSerialize for MintTokens
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for MintTokens
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for MintTokens {
        const DISCRIMINATOR: &'static [u8] = &[59, 132, 24, 246, 122, 39, 8, 243];
    }
    impl anchor_lang::InstructionData for MintTokens {}
    impl anchor_lang::Owner for MintTokens {
        fn owner() -> Pubkey {
            ID
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_collect_fees::*;
    pub use crate::__client_accounts_initialize::*;
    pub use crate::__client_accounts_mint_tokens::*;
    pub use crate::__client_accounts_remove_liquidity::*;
    pub use crate::__client_accounts_add_liquidity::*;
    pub use crate::__client_accounts_initialize_liquidity_pool::*;
    pub use crate::__client_accounts_swap_tokens::*;
}
