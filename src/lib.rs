#![feature(abi_thiscall)]

mod hooks;
mod macros;
mod memory;
mod sdk;
mod utils;

#[cfg(not(target_os = "windows"))]
compile_error!("use windows you fucking degen");

use log::{info, trace};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::ffi::c_void;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::{
    w,
    Win32::{
        Foundation::{BOOLEAN, HINSTANCE},
        System::{
            Console::{AllocConsole, FreeConsole, SetConsoleTitleW},
            LibraryLoader::{
                DisableThreadLibraryCalls, FreeLibraryAndExitThread, GetModuleHandleW,
                GetProcAddress,
            },
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

    sdk::interfaces::init();
    
    let engine_client_ref =
        interface_ref!("VClient018", sdk::interfaces::v_engine_client::EngineClient);
    
    let local_player = engine_client_ref.get_local_player();
    let ingame = engine_client_ref.is_in_game();
    let connected = engine_client_ref.is_connected();
    let max_clients = engine_client_ref.get_max_clients();
    let steam_api_context = engine_client_ref.get_steam_api_context();

    info!("local player id: {local_player:?}");
    info!("ingame: {ingame:?}");
    info!("connected: {connected:?}");
    info!("max clients: {max_clients:?}");
    info!("steam api context: {steam_api_context:#?}");
    

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
