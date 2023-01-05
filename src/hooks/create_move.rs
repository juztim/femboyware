use crate::get_interface;
use crate::sdk::structs::user_cmd::UserCmd;
use log::trace;
use std::ffi::{c_float, c_void};
use byte_strings::c;

type FnCreateMove =
    unsafe extern "fastcall" fn(*const c_void, *const c_void, c_float, *mut UserCmd) -> bool;

pub unsafe fn init() -> bool
{
    let client = get_interface(c!("VClient018"), "client.dll".into());

    let client_mode =
        **(((*((*(client as *mut *mut usize)).offset(10))) + 5) as *mut *mut *mut usize);

    trace!("client mode: {client_mode:?}");

    true
}
