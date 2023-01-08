use crate::sdk::interfaces::v_engine_client::EngineClient;
use crate::utils::get_interface;
use byte_strings::c;
use lazy_static::lazy_static;
use log::info;
use spin::RwLock;
use std::collections::BTreeMap;

pub mod chcl_client;
pub mod i_client_mode;
pub mod steam;
pub mod v_engine_client;

lazy_static! {
    pub static ref INTERFACES: RwLock<BTreeMap<String, usize>> = RwLock::new(BTreeMap::new());
}

pub unsafe fn init() -> bool
{
    let engine_client = get_interface::<EngineClient>(c!("VEngineClient014"), "engine.dll".into());
    INTERFACES
        .write()
        .insert("VClient018".to_string(), engine_client as usize);

    info!("engine client captured at: {engine_client:?}");

    true
}
