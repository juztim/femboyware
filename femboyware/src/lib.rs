#![feature(abi_thiscall)]
#![feature(ptr_sub_ptr)]

extern crate alloc;

mod hooks;
mod macros;
mod memory;
mod pattern;
mod sdk;
mod utils;

use log::{info, trace};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::ffi::c_void;

use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

use crate::pattern::patterns;
use windows::{
    w,
    Win32::{
        Foundation::{BOOLEAN, HINSTANCE},
        System::{
            Console::{AllocConsole, FreeConsole, SetConsoleTitleW},
            LibraryLoader::{DisableThreadLibraryCalls, FreeLibraryAndExitThread},
            SystemServices::DLL_PROCESS_ATTACH,
            Threading::{CreateThread, THREAD_CREATION_FLAGS},
        },
    },
};

unsafe extern "system" fn dll_main(_lparam: *mut c_void) -> u32
{
    AllocConsole();
    SetConsoleTitleW(w!("femboyware"));

    log_init();

    info!("the gay bomb has been deployed");

    let a = patterns::dwClientState.get().unwrap();

    trace!("client state is {:p}", a);

    sdk::interfaces::init();

    hooks::end_scene::init();
    hooks::create_move::init();

    loop
    {
        if GetAsyncKeyState(35) != 0
        {
            FreeConsole();
            FreeLibraryAndExitThread(std::mem::transmute::<*mut c_void, HINSTANCE>(_lparam), 0);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
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

        _ => (),
    }

    true.into()
}

fn log_init()
{
    TermLogger::init(
        log::LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Always,
    )
    .unwrap();
}
