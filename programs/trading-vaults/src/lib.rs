use anchor_lang::prelude::*;

declare_id!("Eqs2vuTeCMLFhULBgh5f2TDuRCrYFecfgJxzUSGLRt21");

#[program]
pub mod trading_vaults {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
