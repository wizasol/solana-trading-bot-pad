use crate::{
    BONK_BUY_IN_DISC, BONK_INIT_DISC, BonkBuy, BonkBuyParam, BonkfunMIntInfo, MoonBuy,
    MoonBuyParamWrapper, MoonshotMintInfo, PumpfunBuy, PumpfunBuyParam, PumpfunMintInfo,
    RAYDIUM_LAUNCHPAD_PROGRAM_ID, parse_bonk_initialize_params,
};
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use yellowstone_grpc_proto::{
    geyser::{SubscribeUpdate, subscribe_update::UpdateOneof},
    prelude::CompiledInstruction,
}; // or use println! if you’re not using the `log` crate

pub fn extract_transaction_data(
    update: &SubscribeUpdate,
) -> Option<(Vec<Pubkey>, Vec<CompiledInstruction>, String)> {
    // borrow the enum inside update_oneof to avoid move
    let transaction_update = match &update.update_oneof {
        Some(UpdateOneof::Transaction(tx_update)) => tx_update,
        _ => return None,
    };

    // safely get references to nested fields
    let tx_info = transaction_update.transaction.as_ref()?;
    let transaction = tx_info.transaction.as_ref()?;
    let meta = tx_info.meta.as_ref()?;
    let tx_msg = transaction.message.as_ref()?;

    // Assume you already have this:
    let mut account_keys: Vec<Pubkey> = tx_msg
        .account_keys
        .iter()
        .filter_map(|k| Pubkey::try_from(k.as_slice()).ok())
        .collect();

    // Append loaded_writable_addresses
    account_keys.extend(
        meta.loaded_writable_addresses
            .iter()
            .filter_map(|raw| Pubkey::try_from(raw.as_slice()).ok()),
    );

    // Append loaded_readonly_addresses
    account_keys.extend(
        meta.loaded_readonly_addresses
            .iter()
            .filter_map(|raw| Pubkey::try_from(raw.as_slice()).ok()),
    );

    let ixs: Vec<CompiledInstruction> = tx_msg.instructions.clone();

    let signature = &tx_info.signature;
    let tx_id = bs58::encode(signature).into_string();

    Some((account_keys, ixs, tx_id))
}

pub fn trade_info(
    ixs: Vec<CompiledInstruction>,
    account_keys: Vec<Pubkey>,
) -> (
    Option<BonkfunMIntInfo>,
    Option<BonkBuy>,
    Option<BonkBuyParam>,
) {
    let mut bonk_mint: Option<BonkfunMIntInfo> = None;
    let mut bonk_buy: Option<BonkBuy> = None;
    let mut bonk_buy_param: Option<BonkBuyParam> = None;

    ixs.iter().for_each(|ix| {
        if ix.data.starts_with(&BONK_INIT_DISC)
            && (account_keys[ix.program_id_index as usize] == RAYDIUM_LAUNCHPAD_PROGRAM_ID)
        {
            let bonk_mint_data = parse_bonk_initialize_params(&ix.data);

            bonk_mint = Some(bonk_mint_data);
        } else if ix.data.starts_with(&BONK_BUY_IN_DISC)
            && (account_keys[ix.program_id_index as usize] == RAYDIUM_LAUNCHPAD_PROGRAM_ID)
        {
            if ix.accounts.len() < 15 {
                eprintln!("Invalid BonkfunBuy account layout");
                return;
            }

            let bonk_fun_buy = BonkBuy {
                payer: account_keys[ix.accounts[0] as usize],
                authority: account_keys[ix.accounts[1] as usize],
                global_config: account_keys[ix.accounts[2] as usize],
                platform_config: account_keys[ix.accounts[3] as usize],
                pool_state: account_keys[ix.accounts[4] as usize],
                user_base_token: account_keys[ix.accounts[5] as usize],
                user_quote_token: account_keys[ix.accounts[6] as usize],
                base_vault: account_keys[ix.accounts[7] as usize],
                quote_vault: account_keys[ix.accounts[8] as usize],
                base_token_mint: account_keys[ix.accounts[9] as usize],
                quote_token_mint: account_keys[ix.accounts[10] as usize],
                base_token_program: account_keys[ix.accounts[11] as usize],
                quote_token_program: account_keys[ix.accounts[12] as usize],
                event_authority: account_keys[ix.accounts[13] as usize],
                program: account_keys[ix.accounts[14] as usize],
            };

            bonk_buy = Some(bonk_fun_buy);
            bonk_buy_param = Some(BonkBuyParam::deserialize(&mut &ix.data[8..]).unwrap());
        }
    });

    (bonk_mint, bonk_buy, bonk_buy_param)
}
