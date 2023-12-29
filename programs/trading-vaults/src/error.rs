use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided Trader Risk Group account key does not match the vault record.")]
    InvalidTraderRiskGroupKey,
    #[msg("The owner of the Trader Risk Group does not match the vault owner.")]
    InvalidTraderRiskGroupOwner,
    NotADepositor,
    #[msg("The requested withdrawal amount exceeds the depositor's balance.")]
    InvalidWithdrawalAmount,
    #[msg("The vault does not have sufficient liquidity to cover the withdrawal.")]
    InsufficientVaultLiquidity,
    #[msg("Insufficient balance for the operation.")]
    InsufficientBalance,
    MathError,
    InsufficientRemainingBalance,
}