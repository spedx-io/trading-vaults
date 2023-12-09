use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod trader_risk_group;
pub mod vault;
pub use error::ErrorCode;
pub use instructions::*;
pub use trader_risk_group::TraderRiskGroup;
pub use vault::Vault;
pub mod investment_status;
pub mod investor;

declare_id!("Eqs2vuTeCMLFhULBgh5f2TDuRCrYFecfgJxzUSGLRt21");
