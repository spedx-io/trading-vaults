use anchor_lang::prelude::*;
pub mod vault;
pub mod instructions;
pub mod trader_risk_group;
pub mod error;
pub use trader_risk_group::TraderRiskGroup;
pub use instructions::*;
pub use error::ErrorCode;
pub use vault::Vault;

declare_id!("Eqs2vuTeCMLFhULBgh5f2TDuRCrYFecfgJxzUSGLRt21");