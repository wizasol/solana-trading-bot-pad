use bonk_sniper_rust::*;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use yellowstone_grpc_proto::geyser::SubscribeRequestFilterTransactions;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // init external clients or global state
    init_nozomi().await;
    init_zslot().await;
    init_jito().await;

    // ✅ Start blockhash handler loop in background
    tokio::spawn(async {
        loop {
            recent_blockhash_handler(RPC_CLIENT.clone()).await;
        }
    });

    // ✅ Start GRPC subscription and processing
    let mut grpc_client = setup_client_grpc(GRPC_ENDPOINT.to_string(), GRPC_TOKEN.to_string())
        .await
        .expect("Failed to connect to GRPC");

    let (subscribe_tx, subscribe_rx) = grpc_client.subscribe().await.unwrap();

    let subscribe_filter = SubscribeRequestFilterTransactions {
        account_include: vec![
            MOONSHOT_PROGRAM_ID.to_string(),
            PUMP_FUN_PROGRAM_ID.to_string(),
            RAYDIUM_LAUNCHPAD_PROGRAM_ID.to_string(),
        ],
        account_exclude: vec![],
        account_required: vec![],
        vote: Some(false),
        failed: Some(false),
        signature: None,
    };

    send_subscription_request_grpc(subscribe_tx, subscribe_filter)
        .await
        .unwrap();

    if let Err(e) = process_updates_grpc(subscribe_rx).await {
        eprintln!("[GRPC] Error processing updates: {e:?}");
    };

    Ok(())
}
