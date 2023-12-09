use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
pub use anchor_lang::prelude::*;

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum InvestmentStatus {
    NotInitialized,  // vault is not initialized
    VoidedDeposit,   // deposit is voided
    ActiveDeposit,   // deposit is accepted
    PendingWithdraw, // withdraw is pending
    Claimable,       // amount is withdrawable
}

impl Default for InvestmentStatus {
    fn default() -> Self {
        InvestmentStatus::NotInitialized
    }
}
