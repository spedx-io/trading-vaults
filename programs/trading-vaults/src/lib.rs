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
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
}
