use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub mint_token: Pubkey,
    pub fee: u16,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Swap {
    pub swap_id: [u8; 32],
    pub lock_time: u64,
    pub secret_hash: [u8; 32],
    pub(crate) secret_key: [u8; 32],
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub mint_token: Pubkey,
    pub amount: u64,
    pub status: u8, // 0: default, 1: start, 2: end, 3: expired
    pub bump: u8,
}
