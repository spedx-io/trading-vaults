use anchor_lang::prelude::*;

pub enum InvestmentStatus {
    NotInitialized, // investment does not exist
    PendingDeposit, // deposit made is pending
    Active, // investment is active and is in use by vault manager
    PendingWithdraw, // withdraw made is pendin g
    PendingForcedSettlement, // force settlement request is pending
    Claimable // returns are now withdrawable
}

impl Default for InvestmentStatus {
    fn default() -> Self {
        InvestmentStatus::Pending // Default status is pending
    }
}

#[account]
pub struct Investor {
    pub is_initialized: bool, // Indicates whether the investor account is initialized
    pub investment_status: InvestmentStatus, // Current status of the investment
    pub amount: u64, // Amount of investment
    pub returns: u64, // Returns from the investment
    pub owner: Pubkey, // Public key of the investor
    pub vault: Pubkey, // Public key of the vault associated with this investment
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

}