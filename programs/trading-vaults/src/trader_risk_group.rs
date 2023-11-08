use anchor_lang::prelude::*;

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

#[derive(Accounts)]
pub struct InitializeTraderRiskGroup<'info> {
    #[account(init, payer = user, space = TraderRiskGroup::LEN)]
    pub trader_risk_group: Account<'info, TraderRiskGroup>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    //TODO: Add other accounts needed for initialization here
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