use anchor_lang::prelude::*;
use crate::trader_risk_group::TraderRiskGroup;
use std::mem::size_of;

type I80F48 = u64; // Can't find I80F48 :(

#[account]
pub struct Vault {
    pub owner: Pubkey,                    // Owner of the vault
    pub balance: u64,                     // Balance of the vault
    pub trader_risk_group: Pubkey,        // Key of the associated Trader Risk Group
    pub is_vault_initialized: bool,       // Is the trading vault initialized
    pub is_vault_paused: bool,            // Is the trading vault paused from accepting deposits
    pub pending_vault_deposits: u64,      // Amount of pending deposits to the vault
    pub pending_vault_withdrawals: u64,   // Amount of pending withdrawals from the vault
    pub vault_manager: Pubkey,            // Vault manager key
    pub no_of_depositors: u64,            // Number of individual depositors to a vault
    pub aum: I80F48,                      // Total value locked of the vault
    pub performance_fee_pct: I80F48,      // The percentage charged by the vault owner from the profits made
    pub performance_fee_growth: I80F48,   // Total amount of percentage fee accumulated
    pub min_investment_amt: u64,          // Minimum amount of investment required into a vault
    pub vault_token_account: Pubkey,      // Account for the vault's token
    pub vault_owner_pct: I80F48,          // Percentage share of the vault owner in the vault
    pub vault_owner_share: I80F48,        // Share of the vault owner in the vault in $ terms
    //TODO: pub vault_force_settle_info: &ForceSettleInfo, // Force settle info of the vault
    pub trg: TraderRiskGroup,             // Hxro Trader Risk Group
    pub is_vault_settled: bool,           // Indicates if the vault is settled
    pub vault_owner_share_pct: I80F48,    // Owner's percentage share in the vault
}

impl Vault {
    // Constructor for Vault
    pub fn new(
        vault_manager: Pubkey,
        trg: TraderRiskGroup,
        owner: Pubkey,
        balance: u64,
        trader_risk_group: Pubkey,
    ) -> Self {
        Self {
            owner,
            balance,
            trader_risk_group,
            is_vault_initialized: false,
            is_vault_paused: false,
            pending_vault_deposits: 0,
            pending_vault_withdrawals: 0,
            vault_manager,
            no_of_depositors: 0,
            aum: 0, 
            performance_fee_pct: 0,
            performance_fee_growth: 0,
            min_investment_amt: 0,
            vault_token_account: Pubkey::default(),
            vault_owner_pct: 0,
            vault_owner_share: 0,
            // vault_force_settle_info: ForceSettleInfo { /* Initialize fields */ },
            trg,
            is_vault_settled: false, // Initializing new field
            vault_owner_share_pct: 0, // Initializing new field
        }
    }

    // Update the LEN constant as per the new struct size
    pub const LEN: usize = 8 // Discriminator
    + size_of::<Pubkey>() * 4 // For `owner`, `trader_risk_group`, `vault_manager`, `vault_token_account`
    + size_of::<bool>() * 3 // For ``, `is_vault_initialized`, `is_vault_paused`, `is_vault_settled`
    + size_of::<u64>() * 5 // For `balance`, `pending_vault_deposits`, `pending_vault_withdrawals`, `no_of_depositors`, `min_investment_amt`
    + size_of::<I80F48>() * 5; // For `aum`, `performance_fee_pct`, `performance_fee_growth`, `vault_owner_pct`, `vault_owner_share`, `vault_owner_share_pct`
}
