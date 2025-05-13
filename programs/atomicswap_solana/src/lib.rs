use anchor_lang::prelude::*;

pub use context::*;

mod context;
mod error;
mod hash_utils;
mod state;

declare_id!("FrmHjkzF1EJ3S45bx6SWvfyCeUNHGcL7AX8mUsfv4DPo");

#[program]
pub mod atomicswap_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u16) -> Result<()> {
        ctx.accounts.initialize(fee, ctx.bumps.pool)
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        swap_id: [u8; 32],
        lock_time: u64,
        secret_hash: [u8; 32],
        buyer: Pubkey,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.deposit(
            swap_id,
            lock_time,
            secret_hash,
            buyer,
            amount,
            ctx.bumps.swap,
        )
    }

    pub fn redeem(ctx: Context<Redeem>, swap_id: [u8; 32], secret_key: [u8; 32]) -> Result<()> {
        ctx.accounts.redeem(swap_id, secret_key)
    }

    pub fn refund(ctx: Context<Refund>, swap_id: [u8; 32]) -> Result<()> {
        ctx.accounts.refund(swap_id)
    }
}
