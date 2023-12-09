use crate::error::ErrorCode;
use crate::investor::Investor;
use crate::trader_risk_group::InitializeTraderRiskGroup;
use crate::{TraderRiskGroup, Vault};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};

pub mod trading_vaults {
    use super::*;
    use std::str::FromStr;

    // TODO: AFTER THE VAULT DATA STRUCTURE HAS BEEN CHANGED, APPLY THE CHANGES HERE
    pub fn initialize(ctx: Context<Initialize>, initial_balance: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let trg: &mut Account<'_, TraderRiskGroup> = &mut ctx.accounts.trader_risk_group;

        vault.owner = *ctx.accounts.owner.key;
        vault.balance = initial_balance; // TODO: ADD FUNCTIONS FOR THE VAULT OWNER TO ADD INITIAL BALANCE TO VAULT
        vault.is_depositor = false;
        vault.trader_risk_group = *trg.to_account_info().key; // Store TRG address in vault

        // Setup the TRG with appropriate default values or passed-in values
        trg.owner = *ctx.accounts.owner.key;
        trg.market_product_group = Pubkey::default();
        trg.positions = Vec::new();
        trg.open_orders = Vec::new();
        trg.cash_deposits = 0;

        Ok(())
    }

    // TODO: DEPOSIT FUNCTION IS NOT PROPER. NO SPL TOKEN TRANSFERS ARE BEING MADE. RE-WRITE FUNCTION TO PERFORM TRANSFER BETWEEN DEPOSITOR ACCOUNT TO THE VAULT SMART CONTRACT
    // TODO: AFTER THE VAULT STRUCTURE HAS BEEN CHANGED, APPLY CHANGES HERE. FOR EG: ENFORCE THE MIN_REQUIRED_DEPOSIT CONSTRAINT AND VAULT LEADER SHARE.
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.balance += amount;
        vault.is_depositor = true;
        Ok(())
    }
    #[derive(Accounts)]
    pub struct Withdraw<'info> {
        #[account(
            mut,
            has_one = owner,
            // TODO: Add constraint to ensure that only the vault owner can initiate a withdrawal and that the withdrawal is to the TRG associated with this Vault.
            close = owner // Only allow the vault to be closed by the owner, freeing space and recovering SOL
        )]
        pub vault: Account<'info, Vault>,
        /// Include the TRG account to enforce withdrawals to this account only
        #[account(
            mut,
            constraint = trader_risk_group.key() == vault.trader_risk_group @ ErrorCode::InvalidTraderRiskGroupKey,
            constraint = trader_risk_group.owner == vault.owner @ ErrorCode::InvalidTraderRiskGroupOwner
            // The above constraints enforce that the TRG passed in is the same as recorded in the vault and
            // that the TRG owner matches the vault owner's public key.
        )]
        pub trader_risk_group: Account<'info, TraderRiskGroup>,
        #[account(mut)]
        pub owner: Signer<'info>,
        // Add the system program to your Withdraw struct to handle account closure if necessary.
        pub system_program: Program<'info, System>,
        #[account(address = token::ID)]
        pub token_program: Program<'info, Token>,
    }

    // TODO: WITHDRAW FUNCTION IS NOT PROPER. FOR NOW ONLY IMPLEMENT FEATURE FOR DEPOSITORS TO WITHDRAW FROM VAULT, NOT VAULT LEADER TO WITHDRAW FROM VAULT TO TRG, THAT WILL BE
    // TODO: A DIFFERENT FUNCTION. ALSO MAKE CHANGES IN ACCORDANCE WITH CHANGES IN VAULT DATA STRUCTURE
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        // Check if the signer is a depositor with a sufficient balance
        if !vault.is_depositor {
            return Err(ErrorCode::NotADepositor.into()); // Convert ErrorCode into the Error type
        }
        if vault.balance < amount {
            return Err(ErrorCode::InsufficientBalance.into()); // Convert ErrorCode into the Error type
        }

        // Verify that the TRG linked with the vault matches the one provided in context
        if ctx.accounts.trader_risk_group.key() != vault.trader_risk_group {
            return Err(ErrorCode::InvalidTraderRiskGroupKey.into());
        }

        // Enforce the minimum balance rule for vault leaders
        if vault.owner == *ctx.accounts.owner.key {
            // Use `checked_sub` to prevent underflow and `checked_mul` to prevent potential overflow
            let current_balance_minus_withdrawal = vault
                .balance
                .checked_sub(amount)
                .ok_or(ProgramError::Custom(ErrorCode::MathError as u32))?;
            let min_required_balance = vault
                .balance
                .checked_mul(10)
                .ok_or(ProgramError::Custom(ErrorCode::MathError as u32))?
                .checked_div(100)
                .ok_or(ProgramError::Custom(ErrorCode::MathError as u32))?;

            if current_balance_minus_withdrawal < min_required_balance {
                return Err(ErrorCode::InsufficientRemainingBalance.into());
            }
        }

        // TODO: Perform the token transfer via CPI
        let cpi_accounts = Transfer {
            from: vault.to_account_info(),
            to: ctx.accounts.trader_risk_group.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        // let cpi_program: AccountInfo<'_> = ctx.accounts.token_program.to_account_info();
        let cpi_context =
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_context, amount)?;

        // Update the vault balance
        vault.balance = vault
            .balance
            .checked_sub(amount)
            .ok_or(ProgramError::Custom(ErrorCode::MathError as u32))?;

        Ok(())
    }

    pub fn initialize_trader_risk_group(ctx: Context<InitializeTraderRiskGroup>) -> Result<()> {
        let trg_account = &mut ctx.accounts.trader_risk_group;

        // Initialize the account with default or provided values
        trg_account.owner = *ctx.accounts.user.key; // Owner's pubkey
        trg_account.market_product_group =
            Pubkey::from_str("FUfpR31LmcP1VSbz5zDaM7nxnH55iBHkpwusgrnhaFjL").unwrap();
        trg_account.positions = Vec::new(); // Start with an empty vector for positions
        trg_account.open_orders = Vec::new(); // Start with an empty vector for orders
        trg_account.cash_deposits = 0; // Start with zero cash deposits

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = Vault::LEN)] // Ensure space includes TRG pubkey size.
    pub vault: Account<'info, Vault>,
    #[account(init, payer = owner, space = TraderRiskGroup::LEN)] // Allocate space for TRG.
    pub trader_risk_group: Account<'info, TraderRiskGroup>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
}
