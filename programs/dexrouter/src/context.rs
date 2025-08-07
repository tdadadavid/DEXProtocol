use anchor_lang::prelude::*;
use anchor_spl::{
  token::{Mint, Token,TokenAccount}
};
use crate::state::*;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Pool::LEN,
        seeds = [
            b"pool",
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
        ],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub mint_a: Account<'info, Mint>,

    #[account()]
    pub mint_b: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        token::mint = mint_a,
        token::authority = pool_signer
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        token::mint = mint_b,
        token::authority = pool_signer
    )]
    pub vault_b: Account<'info, TokenAccount>,

    /// CHECK: This is the PDA signer
    #[account(
        seeds = [b"pool_signer"],
        bump
    )]
    pub pool_signer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapWeighted<'info> {

  #[account(
    mut,
    seeds = [
      b"pool",
      pool.token_mint_a.as_ref(),
      pool.token_mint_b.as_ref(),
    ],
    bump = pool.bump,  
  )]
  pub pool: Account<'info, Pool>,

  /// The vault holding the actual tokens of `token_a`
  #[account(
      mut,
      constraint = vault_a.owner == pool_signer.key(),
    )]
  pub vault_a: Account<'info, TokenAccount>,

  /// The vault holding the actual tokens of `token_b`
  #[account(
    mut,
    constraint = vault_b.owner == pool_signer.key()
  )]
  pub vault_b: Account<'info, TokenAccount>,

  /// CHECK: Used as signer
  #[account(
    seeds = [b"pool_signer"],
    bump,
  )]
  pub pool_signer: UncheckedAccount<'info>,

  /// The user token A program the PDA would interact with.
  #[account(mut)]
  pub user_token_a: Account<'info, TokenAccount>,

  /// The user token B program the PDA would with.
  #[account(mut)]
  pub user_token_b: Account<'info, TokenAccount>,

  #[account(mut)]
  pub user: Signer<'info>,

  pub token_program: Program<'info, Token>,
}


