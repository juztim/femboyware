#![feature(abi_thiscall)]

mod hooks;
mod memory;
mod sdk;
mod utils;

#[cfg(not(target_os = "windows"))]
compile_error!("use windows you fucking degen");

use std::ffi::{c_void, CStr};
use std::ptr;
use simplelog::{Config, TerminalMode, ColorChoice, TermLogger};
use log::{trace, info};
use windows::{
    core::HSTRING,
    s, w,
    Win32::{
        Foundation::{BOOLEAN, HINSTANCE},
        System::{
            Console::{AllocConsole, SetConsoleTitleW},
            LibraryLoader::{DisableThreadLibraryCalls, GetModuleHandleW, GetProcAddress},
            SystemServices::DLL_PROCESS_ATTACH,
            Threading::{CreateThread, THREAD_CREATION_FLAGS},
        },
    },
};

unsafe fn get_interface<T>(name: &CStr, library: HSTRING) -> *mut T
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

unsafe extern "system" fn dll_main(_lparam: *mut c_void) -> u32
{
    AllocConsole();
    SetConsoleTitleW(w!("femboyware"));
    
    log_init();
    
    info!("the gay bomb has been deployed");

    hooks::create_move::init();

    0
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(module: HINSTANCE, reason: u32, _: usize) -> BOOLEAN
{
    DisableThreadLibraryCalls(module);

    #[allow(clippy::single_match)]
    match reason
    {
        DLL_PROCESS_ATTACH =>
        {
            CreateThread(
                None,
                0,
                Some(dll_main),
                Some(module.0 as *const _),
                THREAD_CREATION_FLAGS::default(),
                None,
            )
            .unwrap();
        }

        _ => ()
    }

    true.into()
}

fn log_init() {
    TermLogger::init(log::LevelFilter::Trace, Config::default(), TerminalMode::Stdout, ColorChoice::Always).unwrap();
}
