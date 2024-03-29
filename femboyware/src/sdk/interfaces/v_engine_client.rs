use crate::sdk::interfaces::steam::SteamAPIContext;
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

// https://gitlab.com/KittenPopo/csgo-2018-source/-/blob/main/public/cdll_int.h

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EngineClient {}

impl EngineClient
{
    #[virtual_index(12)]
    pub fn get_local_player(&self) -> usize {}

    #[virtual_index(20)]
    pub fn get_max_clients(&self) -> usize {}

    #[virtual_index(26)]
    pub fn is_in_game(&self) -> bool {}

    #[virtual_index(27)]
    pub fn is_connected(&self) -> bool {}

    #[virtual_index(185)]
    pub fn get_steam_api_context(&self) -> *const SteamAPIContext {}
}

unsafe impl Send for EngineClient {}
