#[cfg(test)]
mod tests {
    use anchor_lang::AnchorDeserialize;
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::native_token::LAMPORTS_PER_SOL;
    use anchor_project::{constants::*, TokenMetadata};
    use mollusk_svm::{Mollusk, result::InstructionResult};
    use sha2::{Digest, Sha256};
    use solana_address::Address;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    use anchor_project::ID;
    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID.to_bytes());
    const BASE_LAMPORTS: u64 = 10 * LAMPORTS_PER_SOL;

    // I Define this struct, so i can use it to serialize my instruction parameters applying prefix for dynamic types => [4-byte length][actual value]
    #[derive(BorshSerialize, BorshDeserialize)]
    struct InitializeMintAccountArgs {
        name: String,
        symbol: String,
        uri: String,
        supply: u64,
    }

    #[test]
    fn test_token_initialization() {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "anchor_project");
        mollusk_svm_programs_token::token::add_program(&mut mollusk);

        let function_name = String::from("initialize_mint_account");
        let discriminator = compute_discriminator(&function_name);

        // Accounts
        // Get the system program ID
        let (system_program_id, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let (token_program, token_program_account) =
            mollusk_svm_programs_token::token::keyed_account();

        println!("Token program: {}", token_program);

        let owner = Pubkey::new_unique();
        let owner_account = Account::new(100_000_000, 0, &system_program_id);

        let mint = Pubkey::new_unique();

        let (token_metadata_pda, _token_metadata_bump) = Address::find_program_address(
            &[
                &TOKEN_METADATA_SEED.as_bytes(),
                &owner.as_ref(),
                &mint.as_ref(),
            ],
            &PROGRAM_ID,
        );

        /*
        I should define these two accounts as a placeholder

        - Because in anchor they have `init` constraint and if i create them here with the right data, my tests won't pass

            let token_metadata_account =
                Account::new(0, std::mem::size_of::<TokenMetadata>(), &system_program_id);

            - For example if i use this i will get error `Allocate: account Address { address: TFGrfhVQcNRSUrRpP2Z8fWGDMic95vMAzPmDRAqvUhZ, base: None } already in use`
        */
        let mint_account = Account::new(0, 0, &system_program_id); // placeholder
        let token_metadata_account = Account::new(0, 0, &system_program_id); // placeholder
            
        let (mint_authority_pda, _mint_authority_bump) = Address::find_program_address(
            &[b"mint_authority", owner.as_ref(), mint.as_ref()],
            &PROGRAM_ID,
        );

        let mint_authority_account = Account::new(0, 0, &mint_authority_pda);

        // Params
        let name = "TestToken";
        let symbol = "TTK";
        let uri = "https://cdn-icons-png.flaticon.com/512/17978/17978725.png";
        let supply: u64 = 100000;

        // Instruction data consist of [Instruction_discriminator][serialized instruction data, in other words params]
        // Since i need every argument to have 4-byte length prefix for every dynamic arg i must use Borsh for simplification
        let args = InitializeMintAccountArgs {
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: uri.to_string(),
            supply,
        };

        let mut instruction_data = discriminator;
        let args_bytes = borsh::to_vec(&args).expect("Failed to serialize args"); // Apply serialization
        instruction_data.extend_from_slice(&args_bytes);

        // Order matters 
        let accounts = [
            (owner, owner_account),
            (mint, mint_account),
            (token_metadata_pda, token_metadata_account),
            (mint_authority_pda, mint_authority_account),
            (token_program, token_program_account),
            (system_program_id, system_account),
        ];

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &instruction_data,
            vec![
                AccountMeta::new(owner, true),
                AccountMeta::new(mint, true), // accounts being created must be signers
                // (any account that is being created or modified in a way that requires authority must sign the transaction)
                AccountMeta::new(token_metadata_pda, false),
                AccountMeta::new_readonly(mint_authority_pda, false),
                AccountMeta::new_readonly(token_program, false),
                AccountMeta::new_readonly(system_program_id, false),
            ],
        );

        let ix_result: InstructionResult = mollusk.process_instruction(&instruction, &accounts);

        // Now i use the `InstructionResult` to fetch accounts and stuff

        let metadata_account = ix_result.get_account(&token_metadata_pda).expect("Expected Metadata Account");
        let metadata_raw_data_binding = metadata_account.data.clone();
        let metadata_data_slice = metadata_raw_data_binding.as_slice();
        let mut cursor = &metadata_data_slice[8..]; // Skip Anchor Account discriminator it is because of this `8 + TokenMetadata::INIT_SPACE` 8 bytes discriminator upfront

        // Metadata Validation checks after transaction 
        let metadata = TokenMetadata::deserialize(&mut cursor).expect("Failed to deserialize Token Metadata");
        assert_eq!(metadata.name, "TestToken");
        assert_eq!(metadata.symbol, "TTK");
        assert_eq!(metadata.mint.to_bytes(), mint.to_bytes());
        assert_eq!(metadata.creator.to_bytes(), owner.to_bytes());
        assert_eq!(metadata.decimals, 9);
        assert_eq!(metadata.supply, 100_000);
    }

    // #[test]
    // fn test_initialize_liquidity_pool() {
    //     let mollusk = Mollusk::new(&PROGRAM_ID, "/Users/emilemilovroydev/Rust/school-of-solana-ackee/my_projects/program-ERoydev/anchor_project/target/deploy/anchor_project");
    //     let function_name = String::from("initialize_liquidity_pool");
    //     let discriminator = compute_discriminator(&function_name);

    //     // Accounts
    //     let owner_pubkey = Address::new_unique();

    //     // let (pool_pda, pool_bump) = Address::find_program_address(
    //     //     &[LIQUIDITY_POOL_SEEDS.as_bytes(), &owner_pubkey.to_bytes(), ]
    //     //     // token_a_mint.key().as_ref(), token_b_mint.key().as_ref()
    //     //     &PROGRAM_ID
    //     // );

    //     // Instruction data consist of [Instruction_discriminator][serialized instruction data, in other words params]
    //     let instruction_data = discriminator;

    //     let accounts = vec![
    //     ];

    //     let instruction = Instruction::new_with_bytes(PROGRAM_ID, &instruction_data, accounts);
    // }

    // #[test]
    // fn test_swap() {
    //     let mollusk = Mollusk::new(&PROGRAM_ID, "/Users/emilemilovroydev/Rust/school-of-solana-ackee/my_projects/program-ERoydev/anchor_project/target/deploy/anchor_project");
    //     let function_name = String::from("swap");
    //     let discriminator = compute_discriminator(&function_name);

    //     let amount_source: u64 = 23;
    //     // Instruction data consist of [Instruction_discriminator][serialized instruction data, in other words params]
    //     let mut instruction_data = discriminator;
    //     instruction_data.extend_from_slice(&amount_source.to_le_bytes());

    //     let accounts = vec![];

    //     let instruction = Instruction::new_with_bytes(PROGRAM_ID, &instruction_data, accounts);
    // }

    // Util
    fn compute_discriminator<'a>(function_name: &'a str) -> Vec<u8> {
        let anchor_prefix = "global:";
        let hash = Sha256::digest(String::from(anchor_prefix) + function_name);
        let discriminator = hash[..8].to_vec();
        discriminator
    }
}
