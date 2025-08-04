use futures::{SinkExt, StreamExt};
use serde_json::json;
use solana_client::client_error::reqwest;
use solana_relayer_adapter_rust::Tips;
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::Instruction, pubkey::Pubkey,
    system_instruction,
};
use spl_associated_token_account::{
    get_associated_token_address, get_associated_token_address_with_program_id,
    instruction::create_associated_token_account_idempotent,
};
use spl_token::instruction::sync_native;
use std::{
    collections::HashMap,
    ops::{Div, Mul},
    sync::Arc,
};
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::{
    geyser::{SubscribeUpdate, subscribe_update::UpdateOneof},
    tonic::Status,
};

use crate::*;

pub async fn process_updates_grpc<S>(mut stream: S) -> Result<(), Box<dyn std::error::Error>>
where
    S: StreamExt<Item = Result<SubscribeUpdate, Status>> + Unpin,
{
    while let Some(result) = stream.next().await {
        match result {
            Ok(update) => {
                let (account_keys, ixs, tx_id) =
                    if let Some(data) = extract_transaction_data(&update) {
                        data
                    } else {
                        continue;
                    };

                let (bonk_raw_mint, bonk_raw_buy, bonk_raw_buy_param) =
                    trade_info(ixs, account_keys);

                if let (Some(bonk_mint), Some(bonk_buy), Some(bonk_buy_param)) =
                    (bonk_raw_mint, bonk_raw_buy, bonk_raw_buy_param)
                {
                    tokio::spawn(async move {
                        let mut bonk_buy = bonk_buy;

                        if CONFIG.filter.x_check {
                            let response_text = reqwest::get(bonk_mint.clone().base_mint_param.uri)
                                .await
                                .unwrap()
                                .text()
                                .await
                                .unwrap();

                            let is_match = CONFIG
                                .filter
                                .x_filter_list
                                .iter()
                                .any(|filter| response_text.contains(filter));

                            if !is_match {
                                println!(
                                    "[Twitter] Check Failed !\n\t* TX Hash : {}\n\t* DATA : {}",
                                    tx_id, response_text
                                );
                                return ();
                            }
                        };

                        if CONFIG.filter.token_name_check {
                            let is_match = CONFIG
                                .filter
                                .token_name_filter_list
                                .iter()
                                .any(|filter| filter == &bonk_mint.base_mint_param.name);

                            if !is_match {
                                println!(
                                    "[Name] Check Failed !\n\t* TX Hash : {}\n\t* DATA : {}",
                                    tx_id, bonk_mint.base_mint_param.name
                                );
                                return ();
                            }
                        };

                        if CONFIG.filter.dev_buy_check {
                            if !bonk_buy_param.amount_in
                                > (CONFIG.filter.dev_buy_limit.mul(10_f64.powi(9))) as u64
                            {
                                println!(
                                    "[DEV BUY] Check Failed !\n\t* TX Hash : {}\n\t* Dev Buy Limit : {}\t* Current Buy Amount : {}",
                                    tx_id,
                                    CONFIG.filter.dev_buy_limit,
                                    (bonk_buy_param.amount_in as f64).div(10_f64.powi(9))
                                );
                                return ();
                            }
                        };

                        println!(
                            "BONKFUN {}\n{:#?}\n{:#?}\n{:#?}\n",
                            tx_id, bonk_mint, bonk_buy, bonk_buy_param
                        );

                        bonk_buy.payer = *PUBKEY;
                        bonk_buy.user_base_token = get_associated_token_address_with_program_id(
                            &bonk_buy.payer,
                            &bonk_buy.base_token_mint,
                            &bonk_buy.base_token_program,
                        );
                        bonk_buy.user_quote_token = get_associated_token_address_with_program_id(
                            &bonk_buy.payer,
                            &bonk_buy.quote_token_mint,
                            &bonk_buy.quote_token_program,
                        );

                        let create_base_ata = create_associated_token_account_idempotent(
                            &bonk_buy.payer,
                            &bonk_buy.payer,
                            &bonk_buy.base_token_mint,
                            &bonk_buy.base_token_program,
                        );

                        let create_quote_ata = create_associated_token_account_idempotent(
                            &bonk_buy.payer,
                            &bonk_buy.payer,
                            &bonk_buy.quote_token_mint,
                            &bonk_buy.quote_token_program,
                        );

                        let transfer_ix = system_instruction::transfer(
                            &PUBKEY,
                            &bonk_buy.user_quote_token,
                            *BUY_SOL_AMOUNT,
                        );
                        let wrap_ix =
                            sync_native(&spl_token::ID, &bonk_buy.user_quote_token).unwrap();

                        let buy_param = BonkBuyParam {
                            amount_in: *BUY_SOL_AMOUNT,
                            minimum_amount_out: 0,
                            share_fee_rate: 0,
                        };


                    });
                };
            }
            Err(e) => {
                eprintln!("Stream error: {}", e);
            }
        }
    }

    Ok(())
}
