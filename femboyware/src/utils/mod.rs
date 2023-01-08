pub mod math;
pub mod memory;

use log::trace;
use std::{ffi::CStr, ptr};
use windows::{
    core::HSTRING,
    s,
    Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress},
};

pub unsafe fn get_interface<T>(name: &CStr, library: HSTRING) -> *mut T
{
    let handle = GetModuleHandleW(&library).expect("failed to get module handle");

    trace!("handle: {handle:?}");

    let function_address =
        GetProcAddress(handle, s!("CreateInterface")).expect("failed to get function address");

    trace!("function address: {:p}", &function_address);

    type CreateInterfaceFn = extern "C" fn(*const i8, *const i32) -> usize;
    let create_interface: CreateInterfaceFn = std::mem::transmute(function_address);

    create_interface(name.as_ptr(), ptr::null()) as *mut T
}
