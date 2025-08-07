use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub weight_a: u64,
    pub weight_b: u64,
    pub bump: u8,
}

impl Pool {
    /// 4 PublicKey.
    pub const LEN: usize = (32 * 4) + (8*2) + 4;
}