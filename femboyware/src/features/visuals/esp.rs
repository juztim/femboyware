use crate::sdk::interfaces::INTERFACES;
use core::borrow::Borrow;
use winapi::shared::d3d9::{IDirect3DDevice9, LPDIRECT3DDEVICE9};
use winapi::um::winnt::INT;

pub unsafe fn run(device: LPDIRECT3DDEVICE9)
{
    let entity_list = INTERFACES.get().unwrap().entity_list;
    let max_entities = entity_list.get_highest_entity_index();
    for i in 0..max_entities
    {
        let entity = entity_list.get_entity_by_index(i);
    }
}

fn box_esp(device: &IDirect3DDevice9) {}

fn name_esp(device: &IDirect3DDevice9)
{
    // get all entities
    // iterate over first 64 entities
}
