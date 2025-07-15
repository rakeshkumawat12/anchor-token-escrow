use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

declare_id!("24UbSkST9eUGakWTfvj1Z86utWHRcVuwzLJfYgzhBDgQ");

#[program]
pub mod non_custodial_escrow {
    use super::*;
   
    pub fn initialize(ctx: Context<Initialize>, x_amount: u64, y_amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.bump = ctx.bumps.escrow; 
        escrow.authority = ctx.accounts.seller.key();
        escrow.escrow_x_token = ctx.accounts.escrowed_x_tokens.key();
        
        escrow.y_amount = y_amount;
        escrow.y_mint = ctx.accounts.y_mint.key();

        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_x_token.to_account_info(),
            to: ctx.accounts.escrowed_x_tokens.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, x_amount)?;
        Ok(())
    }

    pub fn accept(ctx: Context<Accept>) -> Result<()> {
      
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
             
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.escrowed_x_tokens.to_account_info(),
                    to: ctx.accounts.buyer_x_token.to_account_info(),
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                &[&[
                    "escrow".as_bytes(),
                    ctx.accounts.escrow.authority.as_ref(),
                    &[ctx.accounts.escrow.bump],
                ]],
            ),
            ctx.accounts.escrowed_x_tokens.amount,
        )?;

        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.buyer_y_token.to_account_info(),
                    to: ctx.accounts.seller_y_token.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                },
            ),
            ctx.accounts.escrow.y_amount,
        )?;
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
  
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.escrowed_x_tokens.to_account_info(),
                    to: ctx.accounts.seller_x_token.to_account_info(),
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                &[&[
                    "escrow".as_bytes(),
                    ctx.accounts.escrow.authority.as_ref(),
                    &[ctx.accounts.escrow.bump],
                ]],
            ),
            ctx.accounts.escrowed_x_tokens.amount,
        )?;
        anchor_spl::token::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::CloseAccount {
                account: ctx.accounts.escrowed_x_tokens.to_account_info(),
                destination: ctx.accounts.seller.to_account_info(),
                authority: ctx.accounts.escrow.to_account_info(),
            },
            &[&[
                "escrow".as_bytes(),
                ctx.accounts.seller.key().as_ref(),
                &[ctx.accounts.escrow.bump],
            ]],
        ))?;
        Ok(())
    }
}
#[account]
#[derive(InitSpace)]
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
    pub x_mint: Account<'info, Mint>,
    pub y_mint: Account<'info, Mint>, 
    #[account(mut,constraint=seller_x_token.mint==x_mint.key() && seller_x_token.owner==seller.key())]
    pub seller_x_token: Account<'info, TokenAccount>,

    #[account(init,
        payer=seller,
        space=8+Escrow::INIT_SPACE,
        seeds=["escrow".as_bytes(),seller.key().as_ref()],
        bump)]
    pub escrow: Account<'info, Escrow>, 

    #[account(
        init,
        payer=seller,
        token::mint=x_mint,
        token::authority=escrow)] 
    pub escrowed_x_tokens: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>, 
    pub rent: Sysvar<'info, Rent>,            
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Accept<'info> {
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds=["escrow".as_bytes(),escrow.authority.as_ref()],
        bump = escrow.bump, 
    )]
    escrow: Account<'info, Escrow>, 

    #[account(mut,constraint=escrowed_x_tokens.key()==escrow.escrow_x_token)]

    pub escrowed_x_tokens: Account<'info, TokenAccount>,

    #[account(mut,constraint=seller_y_token.mint==escrow.y_mint)]
    pub seller_y_token: Account<'info, TokenAccount>,

    #[account(mut,constraint=buyer_x_token.mint==escrowed_x_tokens.mint)]
    pub buyer_x_token: Account<'info, TokenAccount>,

    #[account(mut,
        constraint=buyer_y_token.mint==escrow.y_mint,
        constraint =  buyer_y_token.owner==buyer.key())]
    pub buyer_y_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>, 
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    pub seller: Signer<'info>,
 
    #[account(
        mut,
        close = seller, constraint = escrow.authority == seller.key(),
        seeds=["escrow".as_bytes(),escrow.authority.as_ref()],
        bump = escrow.bump,
    )]
    escrow: Account<'info, Escrow>, 

    #[account(mut,constraint = escrowed_x_tokens.key()==escrow.escrow_x_token)]
  
    pub escrowed_x_tokens: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint=seller_x_token.mint==escrowed_x_tokens.mint,
        constraint = seller_x_token.owner==seller.key()
    )]
    seller_x_token: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
}