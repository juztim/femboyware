use lazy_static::lazy_static;
use spin::RwLock;
use core::mem;
use std::ffi::c_void;
use byte_strings::c;
use log::{info, trace};
use windows::core::HRESULT;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;
use crate::hooks::vmt::VMTHook;
use crate::utils::get_interface;

type EndSceneFn = unsafe extern "stdcall" fn(*mut IDirect3DDevice9) -> HRESULT;

lazy_static!(
    static ref END_SCENE_HOOK: RwLock<VMTHook> = RwLock::new(VMTHook::default());
    static ref ORIGINAL_END_SCENE: EndSceneFn = unsafe { mem::transmute(END_SCENE_HOOK.read().og_table[42]) };
);

#[allow(dead_code)]
pub unsafe fn init() {
    let shared_device = get_interface::<c_void>(c!("VEngineClient014"), "engine.dll".into());
    info!("shared device is {:p}", shared_device);
    // let vtable = *(shared_device as *const *const usize);
    // info!("vtable is {:p}", vtable);
    // let device = *(vtable.add(37).add(2) as *const *const IDirect3DDevice9);
    // info!("device is {:p}", device);
    // END_SCENE_HOOK.write().init(device as _);
    // info!("end scene hook initialized");
    // END_SCENE_HOOK.write().hook_func(42, hk_end_scene as _);
    // info!("end scene hook installed");
    
    //lazy_static::initialize(&ORIGINAL_END_SCENE);
}

unsafe fn hk_end_scene(device: *mut IDirect3DDevice9) -> HRESULT {
    info!("EndScene called!");
    
    ORIGINAL_END_SCENE(device)
}