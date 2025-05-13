use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::Pool;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    mint_token: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        space = Pool::DISCRIMINATOR.len() + Pool::INIT_SPACE,
        seeds = [b"pool", mint_token.key().as_ref()],
        bump

    )]
    pool: Account<'info, Pool>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::authority = pool,
        associated_token::mint = mint_token
    )]
    pool_ata: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, fee: u16, bump: u8) -> Result<()> {
        self.pool.set_inner(Pool {
            mint_token: self.mint_token.key(),
            fee,
            bump,
        });

        Ok(())
    }
}
