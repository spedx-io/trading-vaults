use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};
use solana_program::entrypoint::ProgramResult;
pub mod vault;
pub mod error;
pub use error::ErrorCode;
pub use vault::Vault;

declare_id!("Eqs2vuTeCMLFhULBgh5f2TDuRCrYFecfgJxzUSGLRt21");

#[program]
pub mod trading_vaults {
    use std::str::FromStr;
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_balance: u64) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        let trg = &mut ctx.accounts.trader_risk_group;
    
        vault.owner = *ctx.accounts.owner.key;
        vault.balance = initial_balance;
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
    
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
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
    
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        
        // Check if the signer is a depositor with a sufficient balance
        if !vault.is_depositor {
            return Err(ProgramError::Custom(ErrorCode::NotADepositor as u32));
        }
        if vault.balance < amount {
            return Err(ProgramError::Custom(ErrorCode::InsufficientBalance as u32));
        }
        
        // Verify that the TRG linked with the vault matches the one provided in context
        if ctx.accounts.trader_risk_group.key() != vault.trader_risk_group {
            return Err(ProgramError::Custom(ErrorCode::InvalidTraderRiskGroupKey as u32));
        }
        
        // Enforce the minimum balance rule for vault leaders
        if vault.owner == *ctx.accounts.owner.key {
            // Use `checked_sub` to prevent underflow and `checked_mul` to prevent potential overflow
            let current_balance_minus_withdrawal = vault.balance.checked_sub(amount).ok_or(ProgramError::Custom(ErrorCode::MathError as u32))?;
            let min_required_balance = vault.balance.checked_mul(10).ok_or(ProgramError::Custom(ErrorCode::MathError as u32))?.checked_div(100).ok_or(ProgramError::Custom(ErrorCode::MathError as u32))?;
    
            if current_balance_minus_withdrawal < min_required_balance {
                return Err(ProgramError::Custom(ErrorCode::InsufficientRemainingBalance as u32));
            }
        }
        
        // Perform the token transfer via CPI
        // This is a placeholder code to show where and how the transfer should be conducted
        // You need to construct the accounts and signers for your specific token transfer, 
        // replace "token::Transfer" with your actual token program's transfer instruction structure, 
        // and use the correct "ctx.accounts" that refer to the token vault, receiver, and authority.
        let cpi_accounts = Transfer {
            from: vault.to_account_info(),
            to: ctx.accounts.trader_risk_group.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program: AccountInfo<'_> = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_context, amount)?;
    
        // Update the vault balance
        vault.balance = vault.balance.checked_sub(amount).ok_or(ProgramError::Custom(ErrorCode::MathError as u32))?;
    
        Ok(())
    }
    
                  
    pub fn initialize_trader_risk_group(
        ctx: Context<InitializeTraderRiskGroup>,
    ) -> ProgramResult {
        let trg_account = &mut ctx.accounts.trader_risk_group;
    
        // Initialize the account with default or provided values
        trg_account.owner = *ctx.accounts.user.key; // Owner's pubkey
        trg_account.market_product_group = Pubkey::from_str("FUfpR31LmcP1VSbz5zDaM7nxnH55iBHkpwusgrnhaFjL").unwrap();
        trg_account.positions = Vec::new(); // Start with an empty vector for positions
        trg_account.open_orders = Vec::new(); // Start with an empty vector for orders
        trg_account.cash_deposits = 0; // Start with zero cash deposits
        
        Ok(())
    }    
}

#[account]
pub struct TraderRiskGroup {
    pub owner: Pubkey,
    pub market_product_group: Pubkey,
    pub positions: Vec<Position>, // Use a vector to store multiple positions.
    pub open_orders: Vec<OpenOrder>, // Use a vector for open orders.
    pub cash_deposits: u64,
    //TODO: Additional fields as necessary.
}

impl TraderRiskGroup {
    pub const MAX_POSITIONS: usize = 100; 
    pub const MAX_OPEN_ORDERS: usize = 50;

    // Define the size of a Position, including any padding required by your specific data alignment needs
    pub const POSITION_SIZE: usize = 32 + 8 + 8; // Size of asset Pubkey + quantity + entry_price

    // Define the size of an OpenOrder similarly to Position
    pub const OPEN_ORDER_SIZE: usize = 1 + 8 + 8 + 1; // Size of order_type + size + price + state

    // This LEN constant would then encapsulate the space computation based on these sizes
    pub const LEN: usize = 8 + // Discriminator
                            32 + // Owner pubkey size
                            32 + // Market product group pubkey size
                            8 + Self::MAX_POSITIONS * Self::POSITION_SIZE + // Size and space for positions
                            8 + Self::MAX_OPEN_ORDERS * Self::OPEN_ORDER_SIZE + // Size and space for orders
                            8; // Cash deposits field size
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Position {
    pub asset: Pubkey,      // The asset this position is in.
    pub quantity: u64,      // The quantity of the asset.
    pub entry_price: u64,   // The price at which the position was entered.
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OpenOrder {
    // Define the structure of an open order according to the requirements.
    pub order_type: OrderType, // Enum for order type (e.g., limit, market).
    pub size: u64,             // Size of the order.
    pub price: u64,            // Order price.
    pub state: OrderState,     // Enum for the state of the order.
    //TODO: Include additional order details as required.
}

// Enums to represent order types and states might be defined as:
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum OrderType {
    Limit,
    Market,
    // Possibly others, like stop-loss or take-profit.
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum OrderState {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    //TODO: Any other states that might be relevant.
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
pub struct InitializeTraderRiskGroup<'info> {
    #[account(init, payer = user, space = TraderRiskGroup::LEN)]
    pub trader_risk_group: Account<'info, TraderRiskGroup>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    //TODO: Add other accounts needed for initialization here
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
}