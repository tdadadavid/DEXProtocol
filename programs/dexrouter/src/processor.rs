use crate::{context::*, math::get_weighted_amount};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

pub fn initialize_pool_handler(
    ctx: Context<InitializePool>,
    weight_a: u64,
    weight_b: u64,
) -> Result<()> {
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
            pool.token_mint_b.key().as_ref(),
        ],
        ctx.program_id,
    );
    pool.bump = bump;

    Ok(())
}

/// 
/// `token_a_to_b`: flag indicating the transfer direction wether A -> B (true) or B -> A (false)
/// 
pub fn swap_weighted_handler(
    ctx: Context<SwapWeighted>,
    amount_in: u64,
    token_a_to_b: bool,
) -> Result<()> {
    // 1. Get vault's balance
    let vault_a_balance = ctx.accounts.vault_a.amount as u128;
    let vault_b_balance = ctx.accounts.vault_b.amount as u128;

    // 2. Get the tokens weights
    let token_a_weight = ctx.accounts.pool.weight_a as u128;
    let token_b_weight = ctx.accounts.pool.weight_b as u128;

     // 3. Compute output using `getWeightedPool` logic
    let amount_out: u64;
    if token_a_to_b {
        amount_out = get_weighted_amount(
            vault_a_balance,
            vault_b_balance,
            token_a_weight,
            token_b_weight,
            amount_in.into(),
        )?;
    } else {
        amount_out = get_weighted_amount(
            vault_b_balance,
            vault_a_balance,
            token_b_weight,
            token_a_weight,
            amount_in.into(),
        )?;
    }


    // 4. Based on the direction of swap transfer to user/program
    if token_a_to_b {
      // Transfer user token_a into program's vault_a
      transfer_from_user_to_vault_account(
        &ctx.accounts.user_token_a,
        &ctx.accounts.vault_a,
        &ctx.accounts.token_program,
        &ctx.accounts.user,
        amount_in,
      )?;

      // Then transfer from vault_b to user account `b`
      transfer_from_vault_to_user_account(
        &ctx.accounts.vault_b, 
        &ctx.accounts.user_token_b, 
        &ctx.accounts.pool_signer, 
        &ctx.accounts.token_program, 
        ctx.bumps.pool_signer, 
        amount_out
      )?;
    } else {
      // Transfer user token_b into program's vault_b
      transfer_from_user_to_vault_account(
        &ctx.accounts.user_token_b,
        &ctx.accounts.vault_b,
        &ctx.accounts.token_program,
        &ctx.accounts.user,
        amount_in,
      )?;

      // Then transfer from vault_a to user account `a`
      transfer_from_vault_to_user_account(
        &ctx.accounts.vault_a, 
        &ctx.accounts.user_token_a, 
        &ctx.accounts.pool_signer, 
        &ctx.accounts.token_program, 
        ctx.bumps.pool_signer, 
        amount_out
      )?;
    }

    Ok(())
}

pub fn mutil_route_swap(ctx: Context<MultiRouteSwap>, amount_in: u64) -> Result<()> {

  Ok(())
}

/// Helper method to send token from program's vault to user's account
fn transfer_from_vault_to_user_account<'info>(
    vault: &Account<'info, TokenAccount>,
    user_token: &Account<'info, TokenAccount>,
    pool_signer: &UncheckedAccount<'info>,
    token_program: &Program<'info, Token>,
    bumps_pool_signer: u8,
    amount_out: u64
) -> Result<()> {
  let signer_seeds: &[&[u8]] = &[b"pool_signer", &[bumps_pool_signer]];

  let cpi_accounts = Transfer {
    from: vault.to_account_info(),
    to: user_token.to_account_info(),
    authority: pool_signer.to_account_info(),
  };
  let cpi_program = token_program.to_account_info();
  let _ = token::transfer(
    CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts, &[signer_seeds]),
    amount_out,
  );
  Ok(())
}

/// Helper method to send token from user's account to programs vault.
fn transfer_from_user_to_vault_account<'info>(
    user_token: &Account<'info, TokenAccount>,
    vault: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    user: &Signer<'info>,
    amount_in: u64
) -> Result<()> {
  let cpi_accounts = Transfer {
    from: user_token.to_account_info(),
    to: vault.to_account_info(),
    authority: user.to_account_info(),
  };
  let cpi_program = token_program.to_account_info();
  let _ = token::transfer(
    CpiContext::new(cpi_program.clone(), cpi_accounts),
    amount_in,
  );
  Ok(())
}