use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Transfer, TokenAccount,Token};

declare_id!("24UbSkST9eUGakWTfvj1Z86utWHRcVuwzLJfYgzhBDgQ");

#[program]
pub mod non_custodial_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;
        escrow.bump = ctx.bumps.escrow;
        escrow.authority = ctx.accounts.seller.key();
        escrow.escrow_x_token = ctx.accounts.escrow_x_tokens.key();
        escrow.y_amount = y_amount;
        escrow.y_mint = ctx.accounts.y_mint.key();
        Ok(())
    }
}

#[derive(Accounts, InitSpace)]
pub struct Escrow {
    authority: Pubkey,
    escrow_x_token: Pubkey,
    bump: u8,
    y_amount: u64,
    y_mint: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    x_mint: Account<'info, Mint>,
    y_mint: Account<'info, Mint>,

    #[account(
        init,
        payer=seller,
        space=8+Escrow::INIT_SPACE,
        seeds=["escrow".as_bytes(), seller.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer=seller,
        token::mint=x_mint,
        token::authority=escrow
    )]
    pub escrow_x_tokens: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>
}
