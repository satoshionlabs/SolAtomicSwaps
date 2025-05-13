use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::error::SwapError;
use crate::hash_utils;
use crate::state::{Pool, Swap};

#[derive(Accounts)]
#[instruction(swap_id: [u8; 32])]
pub struct Redeem<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    mint_token: Account<'info, Mint>,
    #[account(mut)]
    buyer_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"pool", mint_token.key().as_ref()],
        bump = pool.bump
    )]
    pool: Account<'info, Pool>,
    #[account(
        mut,
        associated_token::authority = pool,
        associated_token::mint = mint_token
    )]
    pool_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"swap", pool.key().as_ref(), swap_id.as_ref()],
        bump = swap.bump
    )]
    swap: Account<'info, Swap>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    time: Sysvar<'info, Clock>,
}

impl<'info> Redeem<'info> {
    pub fn redeem(&mut self, swap_id: [u8; 32], secret_key: [u8; 32]) -> Result<()> {
        require_eq!(self.swap.status, 1, SwapError::CanNotRedeem);
        require!(
            self.swap.secret_hash == hash_utils::sha256(&secret_key),
            SwapError::InvalidSecretKey
        );
        require!(
            self.swap.lock_time >= self.time.unix_timestamp as u64,
            SwapError::InvalidRedeemTime
        );
        require_eq!(
            self.swap.buyer,
            self.buyer_ata.owner,
            SwapError::InvalidBuyerPubkey
        );

        let signer_seeds: [&[&[u8]]; 1] = [&[
            &b"pool"[..],
            self.mint_token.to_account_info().key.as_ref(),
            &[self.pool.bump],
        ]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            Transfer {
                from: self.pool_ata.to_account_info(),
                to: self.buyer_ata.to_account_info(),
                authority: self.pool.to_account_info(),
            },
            &signer_seeds,
        );
        transfer(ctx, self.swap.amount)?;

        self.swap.status = 2;
        self.swap.secret_key = secret_key;

        Ok(())
    }
}
