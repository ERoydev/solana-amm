

# Uniswap formulas

## Compute LP tokens to mint for provider of liquidity in pool

1. If the pools i empty and i have to ming LP for the first provider:

```
LP_minted = sqrt(token_a_qty * token_b_qty)
```

2. If the pools is not empty i should mint LP for the provider using different formula:

```
LP minted = min(
    deposit_token_a * total_lp_supply / reserve_token_a,
    deposit_token_b * total_lp_supply / reserve_token_b
)
```

- Inside add_liquidity.rs you can see the how i use `Newton-Raphson Method` to compute square root on an integer without using floats.