use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use crate::{context::*, math::{get_weighted_amount}};

pub fn initialize_pool_handler(ctx: Context<InitializePool>, weight_a: u64,  weight_b: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    pool.token_mint_a = ctx.accounts.mint_a.key();
    pool.token_mint_b = ctx.accounts.mint_b.key();
    pool.vault_a = ctx.accounts.vault_a.key();
    pool.vault_b = ctx.accounts.vault_b.key();
    pool.weight_a = weight_a;
    pool.weight_b = weight_b;

    let (_signer, bump) = Pubkey::find_program_address(
      &[
        b"pool", 
        pool.token_mint_a.key().as_ref(), 
        pool.token_mint_b.key().as_ref()
      ], 
      ctx.program_id
    );
    pool.bump = bump;
   
    Ok(())
}


pub fn swap_weighted_handler(ctx: Context<SwapWeighted>, amount_in: u64) -> Result<()> {
  // 1. Get vault's balance
  let vault_a_balance = ctx.accounts.vault_a.amount as u128;
  let vault_b_balance = ctx.accounts.vault_b.amount as u128;

  // 2. Get the tokens weights
  let token_a_weight = ctx.accounts.pool.weight_a as u128;
  let token_b_weight = ctx.accounts.pool.weight_b as u128;

  // 3. Compute output using `getWeightedPool` logic
  let amount_out = get_weighted_amount(
    vault_a_balance, 
    vault_b_balance, 
    token_a_weight,
    token_b_weight, 
    amount_in.into()
  )?;

  // 4. Transfer user token_a into program's vault_a
  let cpi_accounts = Transfer {
    from: ctx.accounts.user_token_a.to_account_info(),
    to: ctx.accounts.vault_a.to_account_info(),
    authority: ctx.accounts.user.to_account_info(),
  };
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let _ = token::transfer(
    CpiContext::new(cpi_program.clone(), cpi_accounts), 
    amount_in
  );

  // 5. Transfer from vault_b to user account
  
    // generate signer seed
  let signer_seeds: &[&[u8]] = &[
    b"pool_signer",
    &[ctx.bumps.pool_signer]
  ];

  let cpi_accounts  = Transfer {
    from: ctx.accounts.vault_b.to_account_info(),
    to: ctx.accounts.user_token_b.to_account_info(),
    authority: ctx.accounts.pool_signer.to_account_info()
  };
  let _ = token::transfer(
    CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts, &[signer_seeds]), 
    amount_out
  );

  Ok(())
}