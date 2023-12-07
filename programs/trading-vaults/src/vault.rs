use anchor_lang::prelude::*;
// use crate::error::ErrorCode;
use std::mem::size_of;
use crate::trader_risk_group::TraderRiskGroup; // Importing TraderRiskGroup

type I80F48 = u64;

#[account]
pub struct Vault {
    pub owner: Pubkey,                 // Owner of the vault
    pub balance: u64,                  // Balance of the vault
    pub is_depositor: bool,            // Indicates if the vault is a depositor
    pub trader_risk_group: Pubkey,     // Key of the associated Trader Risk Group
    pub is_vault_initialized: bool, // is the trading vault initialized,
    pub is_vault_paused: bool, // is the trading vault paused from accepting deposits,
    pub pending_vault_deposits: u64, // amount of pending deposits to the vault,
    pub pending_vault_withdrawals: u64, // amount of pending withdrawals from the vault,
    pub vault_manager: Pubkey, // vault manager key,
    pub no_of_depositors: u64, // number of individual depositors to a vault,
    pub aum: I80F48, // total value locked of the vault
    pub performance_fee_pct: I80F48, // the percentage charged by the vault owner from the profits made
    pub performance_fee_growth: I80F48, // total amount of percentage fee accumulated
    pub min_investment_amt: u64, // minimum amount of investment required into a vault,
    pub vault_token_account: Pubkey,
    pub vault_owner_pct: I80F48, // percentage share of the vault owner in the vault
    pub vault_owner_share: I80F48, // share of the vault owner in the vault in $ terms,
    //TODO: pub vault_force_settle_info: &ForceSettleInfo, // force settle info of the vault
    pub trg: TraderRiskGroup, // hxro trg
}


impl Vault {
    // Constructor for Vault
    pub fn new(
        vault_manager: Pubkey, 
        trg: TraderRiskGroup, 
        owner: Pubkey, 
        balance: u64, 
        is_depositor: bool, 
        trader_risk_group: Pubkey
    ) -> Self {
        Self {
            is_vault_initialized: false,
            is_vault_paused: false,
            pending_vault_deposits: 0,
            pending_vault_withdrawals: 0,
            vault_manager,
            no_of_depositors: 0,
            aum: 0, // Assuming you replaced I80F48 with u64
            performance_fee_pct: 0,
            performance_fee_growth: 0,
            min_investment_amt: 0,
            vault_token_account: Pubkey::default(),
            vault_owner_pct: 0,
            vault_owner_share: 0,
            // vault_force_settle_info: ForceSettleInfo { /* Initialize fields */ },
            trg,
            // Initializing new fields
            owner,
            balance,
            is_depositor,
            trader_risk_group,
        }
    }
    pub const LEN: usize = 8 // Discriminator
    + size_of::<Pubkey>() * 2 // For `owner` and `trader_risk_group` (32 bytes each)
    + size_of::<u64>() // For `balance` (8 bytes)
    + size_of::<bool>(); // For `is_depositor` (1 byte)
}
