use crate::sdk::interfaces::INTERFACES;

use winapi::shared::d3d9::{IDirect3DDevice9, LPDIRECT3DDEVICE9};


pub unsafe fn run(_device: LPDIRECT3DDEVICE9)
{
    let entity_list = INTERFACES.get().unwrap().entity_list;
    let max_entities = entity_list.get_highest_entity_index();
    for i in 0..max_entities
    {
        let _entity = entity_list.get_entity_by_index(i);
    }
}

fn box_esp(_device: &IDirect3DDevice9) {}

fn name_esp(_device: &IDirect3DDevice9)
{
    // get all entities
    // iterate over first 64 entities
}
