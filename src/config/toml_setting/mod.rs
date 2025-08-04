use std::fs;

use once_cell::sync::Lazy;
use serde::Deserialize;

pub mod credentials;
pub mod filter_setting;
pub mod trade_settings;

pub use credentials::*;
pub use filter_setting::*;
pub use trade_settings::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub wallet: WalletConfig,
    pub rpc: RpcConfig,
    pub grpc: GrpcConfig,
    pub trade: TradeConfig,
    pub priority_fee: PriorityFeeConfig,
    pub services: ServicesConfig,
    pub filter: FilterSetting,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let content = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    toml::from_str(&content).expect("Failed to parse config.toml")
});
