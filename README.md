# Solana AMM Smart Contract

>⚠️ This project uses Anchor version 0.31.1.

Automated Market Maker (AMM) implementation for Solana, built with the Anchor framework. This project demonstrates core DeFi concepts including constant-product formula, liquidity pools, LP tokens, and fee distribution.

## Smart Contract Features

The Anchor program (`anchor_project/`) implements:

### Core AMM Instructions

1. **`initialize_liquidity_pool`** — Creates a new liquidity pool
   - Initializes Pool PDA with token pair configuration
   - Creates escrow accounts for both tokens
   - Sets up LP token mint
   - Configures fee structure (basis points)

2. **`add_liquidity`** — Deposit tokens and receive LP tokens
   - Transfers tokens from user to pool escrows
   - Calculates LP tokens using:
     - First deposit: `LP = √(amount_a × amount_b)` (Newton-Raphson method)
     - Subsequent: `LP = min(deposit_a × total_lp / reserve_a, deposit_b × total_lp / reserve_b)`
   - Mints LP tokens proportional to pool share
   - Updates pool reserves

3. **`remove_liquidity`** — Burn LP tokens and withdraw share
   - Burns LP tokens from provider
   - Calculates proportional share of pool reserves
   - Transfers tokens back to user
   - Updates pool state

4. **`swap`** — Exchange tokens using constant-product formula
   - Implements AMM formula: `dy = (y × dx) / (x + dx)`
   - Deducts fees before swap calculation
   - Routes fees to fee vault
   - Maintains pool balance via escrow accounts
   - Supports bidirectional swaps (A→B or B→A)

5. **`collect_fees`** — Distribute accumulated fees to LPs

### Token Minting Utilities

- **`initialize_mint_account`** — Create SPL token with metadata
- **`mint_tokens`** — Mint tokens to specified account

## Architecture

### PDA Accounts Structure

Each pool derives the following PDAs:

```
Pool PDA (Main State Account)
├── escrow_token_a_account (TokenAccount PDA)
├── escrow_token_b_account (TokenAccount PDA)
├── fee_vault_token_a (TokenAccount PDA)
├── fee_vault_token_b (TokenAccount PDA)
├── lp_mint (Mint PDA)
└── lp_provider (Per-user data PDA)
```

### Pool State

```rust
pub struct Pool {
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub reserve_a: u128,
    pub reserve_b: u128,
    pub total_lp_supply: u64,
    pub fee_bps: u64,  // e.g., 30 = 0.30%
    // ... escrow accounts, creator, timestamps
}
```

## Getting Started

### Prerequisites

### Build & Test

```bash
cd anchor_project

# Build the program
anchor build

# Run tests
anchor test
```

### Deploy

```bash
# Configure Solana CLI for desired network
solana config set --url devnet

# Deploy
anchor deploy
```

## Repository Structure

```
anchor_project/          # Main Anchor workspace
├── programs/
│   └── anchor_project/
│       └── src/
│           ├── lib.rs              # Program entry points
│           ├── instructions/       # Instruction handlers
│           │   ├── initialize_pool.rs
│           │   ├── add_liquidity.rs
│           │   ├── remove_liquidity.rs
│           │   └── swap.rs
│           ├── state/
│           │   └── pool.rs         # Pool account structure
│           ├── errors.rs           # Custom errors
│           ├── events.rs           # Emitted events
│           └── FORMULAS.md         # AMM math documentation
├── tests/               # Integration tests
└── target/idl/          # Generated IDL for client integration

frontend/                # React UI (uses Solana dApp scaffold template*)
backend/                 # Rust websocket service for event monitoring
```

> **Note:** The frontend uses the official [Solana Templates](https://solana.com/developers/templates/react-vite) template provided by Solana Labs for rapid development and integration testing.

## Key Concepts Implemented

### Constant Product Formula
The swap mechanism maintains the invariant `x × y = k` where:
- `x` = reserve of token A
- `y` = reserve of token B  
- `k` = constant product

### Fee Mechanism
- Configurable fee in basis points (e.g., 30 bps = 0.30%)
- Fees deducted before swap calculation
- Accumulated in separate fee vault PDAs
- Distributed proportionally to LP token holders

### LP Token Calculation
- **Initial liquidity:** Geometric mean `√(a × b)`
- **Additional liquidity:** Proportional to existing pool share
- Uses integer square root via Newton-Raphson method (no floating point)


## Additional Documentation

For more detailed implementation notes and instruction data formats, see:
- [docs/instruction_data.md](anchor_project/docs/instruction_data.md)
- [FORMULAS.md](anchor_project/programs/anchor_project/src/FORMULAS.md)

---