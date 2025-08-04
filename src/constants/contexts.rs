use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Copy)]
pub struct PumpfunBuy {
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub creator_vault: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

#[derive(Debug, BorshDeserialize, Clone, Copy)]
pub struct PumpfunBuyParam {
    pub amount: u64,
    pub max_sol_cost: u64,
}

#[derive(Debug, Clone)]
pub struct PumpfunMintInfo {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: Pubkey,
}

#[derive(Debug, Clone, Copy)]
pub struct BonkBuy {
    pub payer: Pubkey,               // #1
    pub authority: Pubkey,           // #2
    pub global_config: Pubkey,       // #3
    pub platform_config: Pubkey,     // #4
    pub pool_state: Pubkey,          // #5
    pub user_base_token: Pubkey,     // #6
    pub user_quote_token: Pubkey,    // #7
    pub base_vault: Pubkey,          // #8
    pub quote_vault: Pubkey,         // #9
    pub base_token_mint: Pubkey,     // #10
    pub quote_token_mint: Pubkey,    // #11
    pub base_token_program: Pubkey,  // #12
    pub quote_token_program: Pubkey, // #13
    pub event_authority: Pubkey,     // #14
    pub program: Pubkey,             // #15
}

#[derive(Debug, BorshDeserialize, Clone, Copy)]
pub struct BonkBuyParam {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub share_fee_rate: u64,
}

#[derive(Debug, Clone)]
pub struct BonkMintParams {
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Debug, Clone)]
pub struct BonkConstantCurve {
    pub supply: u64,
    pub total_base_sell: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Debug, Clone)]
pub struct BonkFixedCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Debug, Clone)]
pub struct BonkLinearCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Debug, Clone)]
pub enum BonkCurveParams {
    Constant(BonkConstantCurve),
    Fixed(BonkFixedCurve),
    Linear(BonkLinearCurve),
    // Add other curve types if needed
}

#[derive(Debug, Clone)]
pub struct BonkVestingParams {
    pub total_locked_amount: u64,
    pub cliff_period: u64,
    pub unlock_period: u64,
}

#[derive(Debug, Clone)]
pub struct BonkfunMIntInfo {
    pub base_mint_param: BonkMintParams,
    pub curve_param: BonkCurveParams,
    pub vesting_param: BonkVestingParams,
}

#[derive(Debug, Clone)]
pub struct MoonBuy {
    pub sender: Pubkey,                   // #1 - Sender
    pub sender_token_account: Pubkey,     // #2 - Sender Token Account
    pub curve_account: Pubkey,            // #3 - Curve Account
    pub curve_token_account: Pubkey,      // #4 - Curve Token Account
    pub dex_fee: Pubkey,                  // #5 - Dex Fee
    pub helio_fee: Pubkey,                // #6 - Helio Fee
    pub mint: Pubkey,                     // #7 - Mint
    pub config_account: Pubkey,           // #8 - Config Account
    pub token_program: Pubkey,            // #9 - Token Program
    pub associated_token_program: Pubkey, // #10 - Associated Token Program
    pub system_program: Pubkey,           // #11 - System Program
}

#[derive(Debug, BorshDeserialize, Clone, Copy)]
pub struct MoonBuyParam {
    pub token_amount: u64,
    pub collateral_amount: u64,
    pub fixed_side: u8,
    pub slippage_bps: u64,
}

#[derive(Debug, BorshDeserialize, Clone)]
pub struct MoonBuyParamWrapper {
    pub data: MoonBuyParam,
}

#[derive(Debug, Clone)]
pub struct MoonshotMintInfo {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub collateral_currency: u8, // 0 = USDC, 1 = USDT, etc.
    pub amount: u64,             // Total token supply
    pub curve_type: u8,          // 0=Constant, 1=Fixed, 2=Linear
    pub migration_target: u8,    // 0=None, 1=AMM, 2=CPSWAP
}
