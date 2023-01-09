use crate::hooking::vmt::VmtHook;
use crate::pattern::patterns;

use log::info;

use std::cell::OnceCell;
use std::ffi::c_void;
use winapi::shared::d3d9::LPDIRECT3DDEVICE9;
use winapi::shared::d3d9types::{D3DCLEAR_TARGET, D3DCOLOR_XRGB, D3DRECT};

use windows::core::HRESULT;

type EndSceneFn = unsafe extern "stdcall" fn(LPDIRECT3DDEVICE9) -> HRESULT;

static mut END_SCENE_HOOK: OnceCell<VmtHook<EndSceneFn>> = OnceCell::new();

#[allow(dead_code)]
pub unsafe fn hook()
{
    let device_ptr = *patterns::DIRECTX_DEVICE.get().unwrap();

    END_SCENE_HOOK
        .set(VmtHook::new(device_ptr as _, 42))
        .unwrap();

    END_SCENE_HOOK.get_mut().unwrap().hook(hk_end_scene);
}

unsafe extern "stdcall" fn hk_end_scene(device: LPDIRECT3DDEVICE9) -> HRESULT
{
    let rect = D3DRECT {
        x1: 30,
        y1: 30,
        x2: 230,
        y2: 230,
    };

    (*device).Clear(
        1,
        &rect as *const D3DRECT,
        D3DCLEAR_TARGET,
        D3DCOLOR_XRGB(255, 0, 0),
        0f32,
        0,
    );

    info!("EndScene Hook called!");
    
    END_SCENE_HOOK.get().unwrap().get_original().unwrap()(device)
}

pub unsafe fn unhook() {
    END_SCENE_HOOK.get_mut().unwrap().unhook();
}