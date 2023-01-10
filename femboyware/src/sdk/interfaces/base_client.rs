use crate::sdk::structs::client_class::ClientClass;
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

// https://gitlab.com/KittenPopo/csgo-2018-source/-/blob/main/game/client/iclientmode.h

#[has_vtable]
#[derive(VTable, Debug)]
pub struct BaseClient {}

impl BaseClient
{
    #[virtual_index(10)]
    pub fn hud_process_input(&self, active: bool) {}

    #[virtual_index(8)]
    pub fn get_all_classes(&self) -> *const ClientClass {}
}
