use crate::hooking::vmt::VmtHook;
use crate::pattern::patterns;

use log::info;

use std::cell::OnceCell;
use std::ffi::c_void;

use windows::core::HRESULT;

type EndSceneFn = unsafe extern "stdcall" fn(*mut c_void) -> HRESULT;

static mut END_SCENE_HOOK: OnceCell<VmtHook<EndSceneFn>> = OnceCell::new();

#[allow(dead_code)]
pub unsafe fn init()
{
    let device_ptr = *patterns::DIRECTX_DEVICE.get().unwrap();

    END_SCENE_HOOK
        .set(VmtHook::new(device_ptr as _, 42))
        .unwrap();

    END_SCENE_HOOK.get_mut().unwrap().hook(hk_end_scene);
}

unsafe extern "stdcall" fn hk_end_scene(device: *mut c_void) -> HRESULT
{
    info!("EndScene called!");

    END_SCENE_HOOK.get().unwrap().get_original().unwrap()(device)
}
