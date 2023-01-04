#[cfg(not(target_os = "windows"))]
compile_error!("use windows you fucking degen");
use std::ffi::c_char;
use eframe::egui;

unsafe fn get_interface<T>(name: *const c_char, library: *const c_char) -> T {
    let handle = GetModuleHandleA(library as *const i8);
    
    let function_address = GetProcAddress(handle, "CreateInterface".as_ptr() as *const i8);
    
    if function_address == std::ptr::null_mut() {
        panic!("Failed to get function address");
    }
    
    type CreateInterfaceFn = extern "C" fn(*const c_char, *mut i32) -> *mut c_void;
    let create_interface: CreateInterfaceFn = std::mem::transmute(function_address);
    create_interface(name.as_ptr() as *const i8, std::ptr::null_mut()) as T
}

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
unsafe extern "system" fn dll_main(_module: *mut c_void) -> u32 {
    AllocConsole();
    SetConsoleTitleA("femboyware".as_ptr() as *const i8);

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(500.0, 500.0)),
        ..Default::default()
    };
    
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