#![feature(abi_thiscall)]

#[cfg(not(target_os = "windows"))]
compile_error!("use windows you fucking degen");

use std::ffi::{c_char, CStr, CString};
use winapi::{
    ctypes::c_void,
    shared::minwindef::{BOOL, DWORD, HMODULE, LPVOID, TRUE},
    um::{
        consoleapi::AllocConsole, libloaderapi::DisableThreadLibraryCalls,
        processthreadsapi::CreateThread, wincon::SetConsoleTitleA, winnt::DLL_PROCESS_ATTACH,
        libloaderapi::GetModuleHandleA,
        libloaderapi::GetProcAddress,
    },
};
use crate::sdk::interfaces::i_client_mode::i_client_mode;

mod sdk;
mod utils;
mod hooks;
mod memory;

unsafe fn get_interface<T>(name: &str, library: &str) -> *mut T {
    let handle = GetModuleHandleA(CString::new(library).unwrap().as_ptr());
    
    if handle.is_null() {
        panic!("failed to get module handle");
    }
    
    println!("handle: {:?}", handle);
    
    let function_address = GetProcAddress(handle, CString::new("CreateInterface").unwrap().as_ptr());
    
    if function_address == std::ptr::null_mut() {
        panic!("Failed to get function address");
    }
    
    println!("function address: {:?}", function_address);
    
    type CreateInterfaceFn = extern "C" fn(*const c_char, *mut i32) -> *mut c_void;
    let create_interface: CreateInterfaceFn = std::mem::transmute(function_address);

    create_interface(CString::new(name).unwrap().as_ptr(), std::ptr::null_mut()) as *mut T
}

unsafe extern "system" fn dll_main(_module: *mut c_void) -> u32 {
    AllocConsole();
    SetConsoleTitleA(CString::new("femboyware").unwrap().as_ptr() as *const i8);
    
    hooks::create_move::init();
    
    0
}

#[allow(unused_variables, non_snake_case)]
#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(module: HMODULE, reason: DWORD, _: LPVOID) -> BOOL {
    DisableThreadLibraryCalls(module);
    if reason != DLL_PROCESS_ATTACH {
        return TRUE;
    }
    CreateThread(
        std::ptr::null_mut(),
        0,
        Some(dll_main),
        module as *mut _,
        0,
        std::ptr::null_mut(),
    );
    TRUE
}