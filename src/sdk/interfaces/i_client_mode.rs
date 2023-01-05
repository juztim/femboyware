use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

// https://gitlab.com/KittenPopo/csgo-2018-source/-/blob/main/game/client/iclientmode.h

#[has_vtable]
#[derive(VTable, Debug)]
pub struct i_client_mode {}

#[allow(non_snake_case)]
impl i_client_mode
{
    #[virtual_index(24)]
    pub fn CreateMove(&self, input_sample_time: f32, c_user_cmd: crate::sdk::structs::user_cmd::user_cmd) -> bool {}
}