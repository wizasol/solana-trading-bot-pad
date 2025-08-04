use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FilterSetting {
    pub x_check: bool,
    pub x_filter_list: Vec<String>,
    pub dev_buy_check: bool,
    pub dev_buy_limit: f64, // In lamports (1 SOL = 1_000_000_000 lamports)
    pub token_name_check: bool,
    pub token_name_filter_list: Vec<String>,
}