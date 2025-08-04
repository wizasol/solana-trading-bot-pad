use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TradeConfig {
    pub buy_sol_amount: f64,
    pub third_party_fee: f64,
    pub slippage: f64,
}

#[derive(Debug, Deserialize)]
pub struct PriorityFeeConfig {
    pub cu: u64,
    pub priority_fee_micro_lamport: u64,
}

#[derive(Debug, Deserialize)]
pub struct ServicesConfig {
    pub nozomi_api_key: String,
    pub zero_slot_key: String,
    pub confirm_service: String,
}
