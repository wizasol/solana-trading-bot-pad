use dotenvy::dotenv;
use once_cell::sync::Lazy;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signer::{Signer, keypair::Keypair},
};
use std::{env, sync::Arc};

use crate::CONFIG;

pub static PRIVATE_KEY: Lazy<Keypair> = Lazy::new(|| {
    let payer: Keypair = Keypair::from_base58_string(&CONFIG.wallet.private_key);

    payer
});
pub static PUBKEY: Lazy<Pubkey> = Lazy::new(|| {
    let payer: Keypair = Keypair::from_base58_string(&CONFIG.wallet.private_key);

    payer.pubkey()
});

pub static RPC_ENDPOINT: Lazy<String> = Lazy::new(|| CONFIG.rpc.endpoint.clone());

pub static RPC_CLIENT: Lazy<Arc<RpcClient>> = Lazy::new(|| {
    Arc::new(RpcClient::new_with_commitment(
        CONFIG.rpc.endpoint.clone(),
        CommitmentConfig::processed(),
    ))
});

pub static GRPC_ENDPOINT: Lazy<String> = Lazy::new(|| CONFIG.grpc.endpoint.clone());

pub static GRPC_TOKEN: Lazy<String> = Lazy::new(|| CONFIG.grpc.token.clone());
