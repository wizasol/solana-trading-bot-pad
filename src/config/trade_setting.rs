use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::CONFIG;

pub static CONFIRM_SERVICE: Lazy<String> = Lazy::new(|| CONFIG.services.confirm_service.clone());

pub static PRIORITY_FEE: Lazy<(u64, u64, f64)> = Lazy::new(|| {
    let cu = CONFIG.priority_fee.cu;
    let priority_fee_micro_lamport = CONFIG.priority_fee.priority_fee_micro_lamport;

    let third_party_fee = CONFIG.trade.third_party_fee;

    (cu, priority_fee_micro_lamport, third_party_fee)
});

pub static BUY_SOL_AMOUNT: Lazy<u64> = Lazy::new(|| {
    let buy_sol_amount = CONFIG.trade.buy_sol_amount;

    (buy_sol_amount * 10_f64.powf(9.0)) as u64
});

pub static SLIPPAGE: Lazy<f64> = Lazy::new(|| {
    let slippage = CONFIG.trade.slippage;

    slippage / 100.0 // convert percent to decimal (e.g., 1.0 -> 0.01)
});
