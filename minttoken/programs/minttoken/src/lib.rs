// src/lib.rs
use anchor_lang::prelude::*;

declare_id!("APES11111111111111111111111111111111111111");

#[program]
mod my_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, supply: u64, decimals: u8) -> ProgramResult {
        let mint = &mut ctx.accounts.mint;
        mint.mint_authority = COption::Some(ctx.accounts.authority.key());
        mint.supply = supply;
        mint.decimals = decimals;
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> ProgramResult {
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;
        let authority = &ctx.accounts.authority;

        // Perform transfer logic here

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 72)]
    pub mint: Account<'info, Mint>,
    pub authority: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, has_one = authority, has_one = mint)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut, has_one = mint)]
    pub to: Account<'info, TokenAccount>,
    pub authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[account]
pub struct Mint {
    pub mint_authority: COption<Pubkey>,
    pub supply: u64,
    pub decimals: u8,
}

#[account]
pub struct TokenAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}