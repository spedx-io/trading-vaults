use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;

declare_id!("Eqs2vuTeCMLFhULBgh5f2TDuRCrYFecfgJxzUSGLRt21");

#[program]
pub mod trading_vaults {
    use std::str::FromStr;
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_balance: u64) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        vault.owner = *ctx.accounts.owner.key;
        vault.balance = initial_balance;
        vault.is_depositor = false;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        vault.balance += amount;
        vault.is_depositor = true;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        if !vault.is_depositor {
            return Err(ProgramError::Custom(2)); // Not a depositor
        }
        if vault.balance >= amount {
            vault.balance -= amount;
            Ok(())
        } else {
            Err(ProgramError::Custom(1)) // Insufficient balance
        }
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
    #[account(init, payer = owner, space = 8 + 32 + 8 + 1)]
    pub vault: Account<'info, Vault>,
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

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
    pub is_depositor: bool,
    pub trader_risk_group: Pubkey, 
}