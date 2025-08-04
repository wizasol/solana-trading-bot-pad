use futures::SinkExt;

use std::collections::HashMap;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterTransactions,
};

pub async fn setup_client_grpc(
    grpc_endpoint: String, // Accepts &str directly, no need for &String
    x_token: String,       // Same here, accepts &str directly
) -> Result<GeyserGrpcClient<impl Interceptor>, Box<dyn std::error::Error>> {
    // Build the gRPC client with TLS config
    let client = GeyserGrpcClient::build_from_shared(grpc_endpoint.to_string())?
        .x_token(Some(x_token.to_string()))?
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;

    Ok(client)
}
/// Send the subscription request with transaction filters
pub async fn send_subscription_request_grpc<T>(
    mut tx: T,
    subscribe_args: SubscribeRequestFilterTransactions,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: SinkExt<SubscribeRequest> + Unpin,
    <T as futures::Sink<SubscribeRequest>>::Error: std::error::Error + 'static,
{
    // Create account filter with the target accounts
    let mut accounts_filter = HashMap::new();
    accounts_filter.insert("account_monitor".to_string(), subscribe_args);

    // Send subscription request
    tx.send(SubscribeRequest {
        transactions: accounts_filter,
        commitment: Some(CommitmentLevel::Processed as i32),
        ..Default::default()
    })
    .await?;

    Ok(())
}
