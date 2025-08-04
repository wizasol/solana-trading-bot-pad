use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WalletConfig {
    pub private_key: String,
}

#[derive(Debug, Deserialize)]
pub struct RpcConfig {
    pub endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct GrpcConfig {
    pub endpoint: String,
    pub token: String,
}