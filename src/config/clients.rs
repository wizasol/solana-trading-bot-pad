use dotenvy::dotenv;
use solana_relayer_adapter_rust::{Jito, Nozomi, ZeroSlot};
use std::env;
use tokio::sync::OnceCell;

use crate::CONFIG;

pub static NOZOMI_CLIENT: OnceCell<Nozomi> = OnceCell::const_new();
pub static ZSLOT_CLIENT: OnceCell<ZeroSlot> = OnceCell::const_new();
pub static JITO_CLIENT: OnceCell<Jito> = OnceCell::const_new();

pub async fn init_nozomi() {
    let nozomi_api_key = CONFIG.services.nozomi_api_key.clone();

    let nozomi = Nozomi::new_auto(nozomi_api_key).await;
    nozomi.health_check(50);
    NOZOMI_CLIENT.set(nozomi).unwrap();
}

pub async fn init_zslot() {
    let zslot_api_key = CONFIG.services.zero_slot_key.clone();

    let zslot = ZeroSlot::new_auto(zslot_api_key).await;
    ZSLOT_CLIENT.set(zslot).unwrap();
}

pub async fn init_jito() {
    let jito = Jito::new_auto(None).await;
    JITO_CLIENT.set(jito).unwrap();
}
