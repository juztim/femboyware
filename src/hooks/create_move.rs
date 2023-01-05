use std::ffi::{c_float, c_void};
use crate::get_interface;
use lazy_static::lazy_static;
use crate::sdk::structs::user_cmd::user_cmd;

type FnCreateMove = unsafe extern "fastcall" fn(*const c_void, *const c_void, c_float, *mut user_cmd) -> bool;

pub unsafe fn init() -> bool {
    let client = get_interface("VClient018", "client.dll");

    let client_mode = **(((*((*(client as *mut *mut usize)).offset(10))) + 5) as *mut *mut *mut usize);
    
    println!("client mode: {:?}", client_mode);
    
    true
}