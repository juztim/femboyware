use crate::sdk::interfaces::base_client::BaseClient;
use crate::sdk::interfaces::entity_list::EntityList;
use crate::sdk::interfaces::v_engine_client::EngineClient;
use crate::utils::get_interface;
use byte_strings::c;
use lazy_static::lazy_static;
use log::info;
use spin::RwLock;
use std::cell::OnceCell;
use std::collections::BTreeMap;
use std::ffi::c_void;

pub mod base_client;
pub mod entity;
pub mod entity_list;
pub mod i_client_mode;
pub mod steam;
pub mod v_engine_client;

#[derive(Debug)]
pub struct Interfaces<'a>
{
    pub engine_client: &'a EngineClient,
    pub entity_list: &'a EntityList,
    pub v_client: &'a BaseClient,
}

/*lazy_static! {
    pub static ref INTERFACES: RwLock<BTreeMap<String, usize>> = RwLock::new(BTreeMap::new());
}*/

pub static mut INTERFACES: OnceCell<Interfaces> = OnceCell::new();

pub unsafe fn init() -> bool
{
    let engine_client = get_interface::<EngineClient>(c!("VEngineClient014"), "engine.dll".into());
    info!("engine client captured at: {engine_client:?}");

    let entity_list = get_interface::<EntityList>(c!("VClientEntityList003"), "client.dll".into());
    info!("entity list captured at: {entity_list:?}");

    let v_client = get_interface::<BaseClient>(c!("VClient018"), "client.dll".into());
    info!("v_client captured at: {v_client:?}");

    let interfaces = Interfaces {
        engine_client: engine_client.as_ref().unwrap(),
        entity_list: entity_list.as_ref().unwrap(),
        v_client: v_client.as_ref().unwrap(),
    };

    INTERFACES.set(interfaces).unwrap();

    // INTERFACES
    //     .write()
    //     .insert("VClient018".to_string(), engine_client as usize);
    //
    // INTERFACES
    //     .write()
    //     .insert("VClientEntityList003".to_string(), entity_list as usize);

    true
}
