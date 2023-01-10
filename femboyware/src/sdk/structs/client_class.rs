use crate::sdk::structs::dt_recv::RecvTable;
use std::ffi::c_char;

type CreateClientClassFn = unsafe extern "system" fn(ent: i32, serial: i32);
type CreateEventFn = unsafe extern "system" fn();

#[derive(Clone)]
#[repr(C)]
pub struct ClientClass
{
    create_client_class: CreateClientClassFn,
    create_event: CreateEventFn,
    network_name: *mut c_char,
    pub recv_table: *mut RecvTable,
    pub next: *mut usize,
    pub class_id: usize,
}
