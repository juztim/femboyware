use crate::sdk::interfaces::entity::BaseEntity;
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EntityList {}

impl EntityList
{
    #[virtual_index(3)]
    pub fn get_entity_by_index(&self, index: i32) -> &'static BaseEntity {}

    #[virtual_index(6)]
    pub fn get_highest_entity_index(&self) -> i32 {}
}
