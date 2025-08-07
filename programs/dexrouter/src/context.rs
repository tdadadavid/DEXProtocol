use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = payer, space = 8 + Pool::LEN)]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RouteSwap<'info> {
    pub pool_a: Account<'info, Pool>,
    pub pool_b: Account<'info, Pool>,
    pub user: Signer<'info>,
}