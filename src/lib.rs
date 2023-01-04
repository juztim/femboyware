#[cfg(not(target_os = "windows"))]
compile_error!("use windows you fucking degen");
use winapi::{
    ctypes::c_void,
    shared::minwindef::{BOOL, DWORD, HMODULE, LPVOID, TRUE},
    um::{
        consoleapi::AllocConsole,
        libloaderapi::DisableThreadLibraryCalls,
        processthreadsapi::CreateThread,
        wincon::SetConsoleTitleA,
        winnt::DLL_PROCESS_ATTACH,
    },
};

unsafe extern "system" fn dll_main(_module: *mut c_void) -> u32
{
    AllocConsole();
    SetConsoleTitleA("rust console\0".as_ptr() as *const i8);
    
    0
}

#[allow(unused_variables, non_snake_case)]
pub unsafe extern "stdcall" fn DllMain(module: HMODULE, reason: DWORD, _: LPVOID) -> BOOL 
{
    DisableThreadLibraryCalls(module);
    if reason != DLL_PROCESS_ATTACH { return TRUE; }
    CreateThread(std::ptr::null_mut(), 0, Some(dll_main), module as *mut _, 0, std::ptr::null_mut());
    TRUE
}