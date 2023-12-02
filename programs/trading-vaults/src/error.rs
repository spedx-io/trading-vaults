use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided Trader Risk Group account key does not match the vault record.")]
    InvalidTraderRiskGroupKey,
    #[msg("The owner of the Trader Risk Group does not match the vault owner.")]
    InvalidTraderRiskGroupOwner,
    NotADepositor,
    #[msg("Insufficient balance for the operation.")]
    InsufficientBalance,
    MathError,
    InsufficientRemainingBalance,
}