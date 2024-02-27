// src/lib.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

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

        // Mint the tokens to the liquidity pool
        token::mint_to(
            &ctx.accounts.token_program.clone(),
            &ctx.accounts.mint.clone(),
            &liquidity_pool.to_account_info().clone(),
            &ctx.accounts.authority.clone(),
            &[&ctx.accounts.authority.clone()],
            liquidity_pool_amount,
        )?;

        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> ProgramResult {
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;
        let authority = &ctx.accounts.authority;
        let marketing_wallet = &mut ctx.accounts.marketing_wallet;
        let king_whale_wallet = &mut ctx.accounts.king_whale_wallet;
        let lock_account = &mut ctx.accounts.lock_account;
        let clock = &ctx.accounts.clock;

        // Check if the authority is the owner of the 'from' account
        if *authority.key != from.owner {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Calculate the tax (5%)
        let tax = amount / 20;
        let king_whale_tax = tax / 5;  // 1% of the tax goes to the king whale
        let marketing_tax = tax - king_whale_tax;  // 4% of the tax goes to the marketing wallet
        let liquidity_pool_tax = amount / 5;  // 20% of the amount goes to the liquidity pool
        let transferred_amount = amount - tax;

        // Update 'from' and 'to' account balances
        from.amount -= amount;
        to.amount += transferred_amount;

        // Identify the king whale and send 1% tax to the king whale
        if amount > king_whale_wallet.highest_buy_amount {
            king_whale_wallet.highest_buy_amount = amount;
            king_whale_wallet.whale_wallet = *from.to_account_info().key;
        }

        if *from.to_account_info().key == king_whale_wallet.whale_wallet {
            from.amount -= king_whale_tax;
            king_whale_wallet.amount += king_whale_tax;
        } else {
            // Send the tax to the marketing wallet (4%)
            from.amount -= marketing_tax;
            marketing_wallet.amount += marketing_tax;
        }

        // Send 20% of the transferred amount to the liquidity pool
        from.amount -= liquidity_pool_tax;
        liquidity_pool.amount += liquidity_pool_tax;

        // Lock 90% of the transferred amount
        let lock_amount = (transferred_amount * 9) / 10;
        from.amount -= lock_amount;
        to.amount += lock_amount;

        // Create a lock account for the locked tokens
        lock_account.amount = lock_amount;
        lock_account.release_time = clock.slot + 24 * 60 * 60 / clock.tick_height; // Unlock after 24 hours

        Ok(())
    }

    pub fn unlock(ctx: Context<Unlock>) -> ProgramResult {
        let lock_account = &mut ctx.accounts.lock_account;
        let to = &mut ctx.accounts.to;

        // Check if it's time to unlock the tokens
        if ctx.accounts.clock.slot >= lock_account.release_time {
            // Unlock the tokens
            to.amount += lock_account.amount;
            lock_account.amount = 0;
            lock_account.release_time = 0;
        }

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
    pub token_program: AccountInfo<'info>,
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
    #[account(init, payer = authority, space = 8 + 32)]
    pub lock_account: Account<'info, LockAccount>,
    pub clock: Sysvar<'info, Clock>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Unlock<'info> {
    #[account(mut)]
    pub lock_account: Account<'info, LockAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct BurnLiquidity<'info> {
    #[account(mut)]
    pub liquidity_pool: Account<'info, TokenAccount>,
}

#[account]
pub struct LockAccount {
    pub amount: u64,
    pub release_time: i64,
}

#[account]
pub struct TokenAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[account]
pub struct Mint {
    pub mint_authority: COption<Pubkey>,
    pub supply: u64,
    pub decimals: u8,
}

#[account]
pub struct KingWhaleWallet {
    pub whale_wallet: Pubkey,
    pub highest_buy_amount: u64,
    pub amount: u64,
}
