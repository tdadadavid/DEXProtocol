pub mod context;
pub mod errors;
pub mod math;
pub mod processor;
pub mod state;

use anchor_lang::prelude::*;
use context::*;
use processor::{initialize_pool_handler, swap_weighted_handler};

declare_id!("4vMwWget8jXVv4LGREx6XBDTs7ZuPhZpcxWcPiGu7jDS");

#[program]
pub mod dexrouter {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        weight_a: u64,
        weight_b: u64,
    ) -> Result<()> {
        initialize_pool_handler(ctx, weight_a, weight_b)
    }

    pub fn swap_weighted(ctx: Context<SwapWeighted>, amount_in: u64) -> Result<()> {
        swap_weighted_handler(ctx, amount_in, false)
    }

    pub fn multi_route_swap(ctx: Context<MultiRouteSwap>, amount_in: u64) -> Result<()> {
        Ok(())
    }
}
