use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

// https://gitlab.com/KittenPopo/csgo-2018-source/-/blob/main/public/cdll_int.h

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EngineClient {}

impl EngineClient
{
    #[virtual_index(185)]
    pub fn get_steam_api_context(&self) -> i32 {}
}

unsafe impl Send for EngineClient {}