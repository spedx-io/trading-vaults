use crate::error::ErrorCode;
use crate::investor::Investor;
use crate::investment_status::InvestmentStatus;
use crate::trader_risk_group::InitializeTraderRiskGroup;
use crate::{TraderRiskGroup, Vault};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};

pub mod trading_vaults {
    use super::*;
    use std::str::FromStr;

    pub fn initialize(ctx: Context<Initialize>, initial_balance: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let trg: &mut Account<'_, TraderRiskGroup> = &mut ctx.accounts.trader_risk_group;

        vault.owner = *ctx.accounts.owner.key;
        vault.balance = initial_balance;
        vault.trader_risk_group = *trg.to_account_info().key;

        trg.owner = *ctx.accounts.owner.key;
        trg.market_product_group = Pubkey::default();
        trg.positions = Vec::new();
        trg.open_orders = Vec::new();
        trg.cash_deposits = 0;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let investor = &mut ctx.accounts.investor;

        // Calculate the new total deposit amount including this deposit
        let new_total_deposit = vault.total_deposits + amount;

        // Calculate the vault manager's share after this deposit
        let vault_manager_share: f64 = (vault.manager_deposits as f64 / new_total_deposit as f64) * 100.0;

        if vault_manager_share < 10.0 {
            // If the deposit shoves the vault manager's share below 10%, void the deposit
            investor.investment_status = InvestmentStatus::VoidedDeposit;
        } else {
            // Accept the deposit
            vault.total_deposits = new_total_deposit;
            investor.amount += amount;
            investor.investment_status = InvestmentStatus::ActiveDeposit;
        }

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let investor = &mut ctx.accounts.investor;

        if investor.amount < amount {
            return Err(ErrorCode::InvalidWithdrawalAmount.into());
        }

        if vault.balance < amount {
            investor.investment_status = InvestmentStatus::PendingWithdraw;
            return Err(ErrorCode::InsufficientVaultLiquidity.into());
        }

        let cpi_accounts = Transfer {
            from: vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_context, amount)?;

        vault.balance = vault
            .balance
            .checked_sub(amount)
            .ok_or(ProgramError::Custom(ErrorCode::MathError as u32))?;
        investor.amount = investor
            .amount
            .checked_sub(amount)
            .ok_or(ProgramError::Custom(ErrorCode::MathError as u32))?;
        investor.investment_status = InvestmentStatus::Claimable;

        Ok(())
    }

    pub fn initialize_trader_risk_group(ctx: Context<InitializeTraderRiskGroup>) -> Result<()> {
        let trg_account = &mut ctx.accounts.trader_risk_group;

        trg_account.owner = *ctx.accounts.user.key;
        trg_account.market_product_group =
            Pubkey::from_str("FUfpR31LmcP1VSbz5zDaM7nxnH55iBHkpwusgrnhaFjL").unwrap();
        trg_account.positions = Vec::new();
        trg_account.open_orders = Vec::new();
        trg_account.cash_deposits = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = Vault::LEN)]
    pub vault: Account<'info, Vault>,
    #[account(init, payer = owner, space = TraderRiskGroup::LEN)]
    pub trader_risk_group: Account<'info, TraderRiskGroup>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub investor: Account<'info, Investor>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub investor: Account<'info, Investor>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub trader_risk_group: Account<'info, TraderRiskGroup>,
    pub system_program: Program<'info, System>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}
