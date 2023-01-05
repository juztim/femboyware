use crate::sdk::interfaces::chcl_client::HclClient;
use lazy_static::lazy_static;
use spin::RwLock;
use std::ffi::c_void;

// lazy_static! {
//     static ref CLIENT_INTERFACE: *const chcl_client = RwLock::new(0x0);
//     pub static ref CLIENT_MODE: *const i_client_mode = unsafe { std::mem::transmute(**(((*((*(CLIENT_INTERFACE as *mut *mut usize)).offset(10))) + 5) as *mut *mut *mut usize)) };
// }

// pub unsafe fn init(client_interface: *const chcl_client) -> bool {
//     CLIENT_INTERFACE.write() = client_interface;
//     true
// }
