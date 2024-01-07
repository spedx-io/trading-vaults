use crate::investment_status::InvestmentStatus;
use anchor_lang::prelude::*;

#[account]
pub struct Investor {
    pub is_initialized: bool, // Indicates whether the investor account is initialized
    pub investment_status: InvestmentStatus, // Current status of the investment
    pub amount: u64,          // Amount of investment
    pub returns: u64,         // Returns from the investment
    pub owner: Pubkey,        // Public key of the investor
    pub vault: Pubkey,        // Public key of the vault associated with this investment
    pub fund_pubkey: Pubkey, // The public key of the associated vault
}

impl Investor {
    // Constructor for Investor
    pub fn new(owner: Pubkey, vault: Pubkey) -> Self {
        Self {
            is_initialized: true,
            investment_status: InvestmentStatus::default(),
            amount: 0,
            returns: 0,
            owner,
            vault,
        }
    }

    // Method to update the investment amount
    pub fn update_investment_amount(&mut self, additional_amount: u64) {
        self.amount += additional_amount;
    }

}
