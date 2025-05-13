use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::state::{Pool, Swap};

#[derive(Accounts)]
#[instruction(swap_id: [u8; 32])]
pub struct Deposit<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    mint_token: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::authority = signer,
        associated_token::mint = mint_token
    )]
    signer_ata: Account<'info, TokenAccount>,
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
        init,
        payer = signer,
        space = Swap::DISCRIMINATOR.len() + Swap::INIT_SPACE,
        seeds = [b"swap", pool.key().as_ref(), swap_id.as_ref()],
        bump
    )]
    swap: Account<'info, Swap>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(
        &mut self,
        swap_id: [u8; 32],
        lock_time: u64,
        secret_hash: [u8; 32],
        buyer: Pubkey,
        amount: u64,
        bump: u8,
    ) -> Result<()> {
        self.swap.set_inner(Swap {
            swap_id,
            lock_time,
            secret_hash,
            secret_key: [0u8; 32],
            seller: self.signer.key(),
            buyer,
            mint_token: self.mint_token.key(),
            amount,
            status: 1,
            bump,
        });

        let ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.signer_ata.to_account_info(),
                to: self.pool_ata.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        );
        transfer(ctx, amount)?;

        Ok(())
    }
}
