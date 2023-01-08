use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

// https://gitlab.com/KittenPopo/csgo-2018-source/-/blob/main/game/client/iclientmode.h

#[has_vtable]
#[derive(VTable, Debug)]
pub struct HclClient {}

impl HclClient
{
    #[virtual_index(10)]
    pub fn hud_process_input(&self, active: bool) {}
}
