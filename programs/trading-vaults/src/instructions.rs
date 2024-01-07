use crate::error::ErrorCode;
use crate::investment_status::InvestmentStatus;
use crate::investor::Investor;
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
    
        // Ensure the investor is depositing into the correct vault
        if investor.fund_pubkey != vault.to_account_info().key() {
            return Err(ErrorCode::InvalidVault.into());
        }
    
        // Calculate the new total deposit amount including this deposit
        let new_total_deposit = vault
            .total_deposits
            .checked_add(amount)
            .ok_or(ErrorCode::MathError)?;
    
        // Calculate the vault manager's share after this deposit
        let vault_manager_share = vault.manager_deposits as f64 / new_total_deposit as f64;
    
        if vault_manager_share < 0.10 {
            // If the deposit reduces the vault manager's share below 10%, void the deposit
            investor.investment_status = InvestmentStatus::VoidedDeposit;
            return Err(ErrorCode::VaultManagerShareTooLow.into());
        } else {
            // Process the SPL token transfer from the investor to the vault
            let cpi_accounts = Transfer {
                from: investor.to_account_info(),
                to: vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            };
            let cpi_context =
                CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            token::transfer(cpi_context, amount)?;
    
            // Accept the deposit
            vault.total_deposits = new_total_deposit;
            if investor.is_initialized {
                // If the investor is already initialized, update the investment amount
                investor.update_investment_amount(amount);
            } else {
                // For a new investor, set the amount and mark as initialized
                investor.amount = amount;
                investor.is_initialized = true;
            }
            investor.investment_status = InvestmentStatus::ActiveDeposit;
        }
    
        Ok(())
    }
    
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let investor = &mut ctx.accounts.investor;

        // Ensure the investor is withdrawing from the correct vault
        if investor.fund_pubkey != vault.to_account_info().key() {
            return Err(ErrorCode::InvalidVault.into());
        }

        // Check if the user is a depositor and has deposited enough
        if investor.amount < amount {
            return Err(ErrorCode::InvalidWithdrawalAmount.into());
        }

        // Check if the vault has enough balance for the withdrawal
        if vault.balance < amount {
            investor.investment_status = InvestmentStatus::PendingWithdraw;
            return Err(ErrorCode::InsufficientVaultLiquidity.into());
        }

        // Process the withdrawal
        let cpi_accounts = Transfer {
            from: vault.to_account_info(),
            to: investor.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_context =
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_context, amount)?;

        // Update balances and statuses
        vault.balance = vault
            .balance
            .checked_sub(amount)
            .ok_or(ErrorCode::MathError)?;
        investor.amount = investor
            .amount
            .checked_sub(amount)
            .ok_or(ErrorCode::MathError)?;
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
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
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
