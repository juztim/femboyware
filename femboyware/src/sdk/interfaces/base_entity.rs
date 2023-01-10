use vtables_derive::{has_vtable, VTable,virtual_index};
use vtables::VTable;
use crate::netvar;
use crate::sdk::interfaces::base_object::BaseObject;

// https://gitlab.com/KittenPopo/csgo-2018-source/-/blob/main/game/client/c_baseentity.h

#[repr(C)]
#[derive(Debug, VTable)]
#[has_vtable]
pub struct BaseEntity {}

impl BaseObject for BaseEntity {}

impl BaseEntity
{
    pub fn health(&self) -> i32
    {
        netvar!(self, "DT_BasePlayer->m_iHealth");
    }
    
    #[virtual_index(156)]
    pub fn is_alive(&self) -> bool {}
    
    #[virtual_index(158)]
    pub fn is_player(&self) -> bool {}
}
