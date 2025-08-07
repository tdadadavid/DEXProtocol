use anchor_lang::prelude::*;
use crate::{state::*, context::*, math::*};

pub fn initialize_pool_handler(ctx: Context<InitializePool>, token_a: u64, token_b: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.token_a = token_a;
    pool.token_b = token_b;
    Ok(())
}

pub fn swap_handler(ctx: Context<Swap>, amount_in: u64) -> Result<()> {
    // get pool
    let pool = &mut ctx.accounts.pool;
    let output = get_output_amount(pool.token_a, pool.token_b, amount_in)?;
    pool.token_a += amount_in;
    pool.token_b -= output;
    msg!("Swapped {} for {}", amount_in, output);
    Ok(())
}

pub fn route_swap_handler(ctx: Context<RouteSwap>, amount_in: u64) -> Result<()> {
    // First route through pool_a
    let pool_a = &mut ctx.accounts.pool_a;
    let intermediate = get_output_amount(pool_a.token_a, pool_a.token_b, amount_in)?;
    pool_a.token_a += amount_in;
    pool_a.token_b -= intermediate;

    // Route through pool_b
    let pool_b = &mut ctx.accounts.pool_b;
    let final_out = get_output_amount(pool_b.token_a, pool_b.token_b, intermediate)?;
    pool_b.token_a += intermediate;
    pool_b.token_b += final_out;

    msg!("Routed {} through 2 pools for {}", amount_in, final_out);

    Ok(())
}