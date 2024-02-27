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
        let marketing_wallet = &ctx.accounts.marketing_wallet;
        let king_whale_wallet = &ctx.accounts.king_whale_wallet;

        // Check if the authority is the owner of the 'from' account
        if *authority.key != from.owner {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Calculate the tax (5%)
        let tax = amount / 20;
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
            let king_whale_tax = amount / 100;
            from.amount -= king_whale_tax;
            king_whale_wallet.amount += king_whale_tax;
        } else {
            // Send the tax to the marketing wallet (4%)
            from.amount -= tax - (amount / 100);
            marketing_wallet.amount += tax - (amount / 100);
        }

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
    pub marketing_wallet: Account<'info, TokenAccount>,
    pub king_whale_wallet: Account<'info, KingWhaleWallet>,
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
