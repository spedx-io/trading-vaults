use anchor_lang::prelude::*;
use crate::error::ErrorCode;
#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
    pub is_depositor: bool,
    pub trader_risk_group: Pubkey, // Field to link to the TRG account
}

impl Vault {
    // Adjusted LEN calculation to reflect the current structure
    pub const LEN: usize = 8 + // Discriminator
                            32 + // Owner Pubkey
                            8 + // Balance
                            1 + // Is Depositor
                            32; // Trader Risk Group Pubkey

    // Method to initialize a new Vault
    pub fn new(owner: Pubkey, trader_risk_group: Pubkey) -> Self {
        Self {
            owner,
            balance: 0, // Initial balance set to 0
            is_depositor: false,
            trader_risk_group,
        }
    }

    // Method to handle deposits
    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
        self.is_depositor = true;
    }

    // Method to handle withdrawals
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        if self.balance < amount {
            return Err(error!(ErrorCode::InsufficientBalance));
        }
        self.balance -= amount;
        Ok(())
    }
}