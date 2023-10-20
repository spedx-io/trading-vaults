use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;

declare_id!("Eqs2vuTeCMLFhULBgh5f2TDuRCrYFecfgJxzUSGLRt21");

#[program]
pub mod trading_vaults {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_balance: u64) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        vault.owner = *ctx.accounts.owner.key;
        vault.balance = initial_balance;
        vault.is_depositor = false;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        vault.balance += amount;
        vault.is_depositor = true;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        if !vault.is_depositor {
            return Err(ProgramError::Custom(2)); // Not a depositor
        }
        if vault.balance >= amount {
            vault.balance -= amount;
            Ok(())
        } else {
            Err(ProgramError::Custom(1)) // Insufficient balance
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 1)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
    pub is_depositor: bool,
}
