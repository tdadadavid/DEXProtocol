pub mod context;
pub mod state;
pub mod processor;
pub mod math;

use anchor_lang::prelude::*;

use context::*;
use processor::*;

declare_id!("4vMwWget8jXVv4LGREx6XBDTs7ZuPhZpcxWcPiGu7jDS");

#[program]
pub mod dexrouter {
    use super::*;

    pub fn initialize(ctx: Context<InitializePool>, token_a_amount: u64, token_b_amount: u64) -> Result<()> {
        initialize_pool_handler(ctx, token_a_amount, token_b_amount)
    }


    pub fn swap(ctx: Context<Swap>, amount_in: u64) -> Result<()> {
        swap_handler(ctx, amount_in)
    }

    pub fn route_swap(ctx: Context<RouteSwap>, amount_in: u64) -> Result<()> {
        route_swap_handler(ctx, amount_in)
    }
}
