use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
    pub is_depositor: bool,
    pub trader_risk_group: Pubkey, // Field to link to the TRG account
}

impl Vault {
    // This calculation must be adjusted to add more fields to the struct
    pub const LEN: usize = 8 + 32 + 8 + 1 + 32; // Add size for trader_risk_group pubkey
}