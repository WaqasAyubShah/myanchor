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
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;
        let authority = &ctx.accounts.authority;
        let marketing_wallet = &mut ctx.accounts.marketing_wallet;
        let king_whale_wallet = &mut ctx.accounts.king_whale_wallet;
        let liquidity_pool = &mut ctx.accounts.liquidity_pool;

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
