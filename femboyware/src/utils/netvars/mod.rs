use crate::sdk::interfaces::INTERFACES;
use crate::sdk::structs::dt_recv::{PropType, RecvTable};
use log::info;
use spin::RwLock;
use std::collections::BTreeMap;
use std::ffi::CStr;


pub static mut NETVARS: RwLock<BTreeMap<String, usize>> = RwLock::new(BTreeMap::new());

pub unsafe fn init()
{
    info!("dumping netvars");
    let v_client = INTERFACES.get().unwrap().v_client;

    let mut client_class = v_client.get_all_classes();

    while !client_class.is_null()
    {
        let recv_table = client_class.read().recv_table;
        write(recv_table);
        client_class = client_class.read().next as _;
    }

    info!("successfully dumped {} netvars", NETVARS.read().len());
}

pub unsafe fn get(name: &str) -> usize
{
    *NETVARS.read().get(name).unwrap()
}

unsafe fn write(table: *mut RecvTable)
{
    let table_name = CStr::from_ptr(table.read().table_name).to_str().unwrap();

    for i in 0..table.read().n_props
    {
        let prop = table.read().p_props.offset(i as _).read();
        let p_name = CStr::from_ptr(prop.prop_name).to_str().unwrap();
        if prop.prop_type == PropType::DataTable
        {
            write(prop.data_table);
        }
        NETVARS.write().insert(
            alloc::format!("{table_name}->{p_name}"),
            prop.offset as _,
        );
    }
}
