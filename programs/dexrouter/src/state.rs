use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub token_a: u64,
    pub token_b: u64
}

impl Pool {
    pub const LEN: usize = 8+8; // token_a + token_b
}