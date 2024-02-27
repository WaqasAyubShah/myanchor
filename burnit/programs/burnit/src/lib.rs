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

        // Calculate the amount to be sent to the liquidity pool (20% of supply)
        let liquidity_pool_amount = supply / 5;

        // Create the liquidity pool account
        let liquidity_pool = &mut ctx.accounts.liquidity_pool;
        liquidity_pool.amount = liquidity_pool_amount;

        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> ProgramResult {
        // ... existing transfer logic ...

        Ok(())
    }

    pub fn burn_liquidity(ctx: Context<BurnLiquidity>) -> ProgramResult {
        let liquidity_pool = &mut ctx.accounts.liquidity_pool;

        // Calculate the amount to burn (1% of total liquidity)
        let burn_amount = liquidity_pool.amount / 100;

        // Burn tokens from the liquidity pool
        liquidity_pool.amount -= burn_amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 72)]
    pub mint: Account<'info, Mint>,
    pub authority: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    #[account(mut, has_one = mint)]
    pub liquidity_pool: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, has_one = authority, has_one = mint)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut, has_one = mint)]
    pub to: Account<'info, TokenAccount>,
    pub authority: AccountInfo<'info>,
    pub marketing_wallet: Account<'info, TokenAccount>,
    pub king_whale_wallet: Account<'info, KingWhaleWallet>,
    pub liquidity_pool: Account<'info, TokenAccount>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BurnLiquidity<'info> {
    #[account(mut)]
    pub liquidity_pool: Account<'info, TokenAccount>,
    pub clock: Sysvar<'info, Clock>,
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

#[account]
pub struct KingWhaleWallet {
    pub whale_wallet: Pubkey,
    pub highest_buy_amount: u64,
    pub amount: u64,
}
