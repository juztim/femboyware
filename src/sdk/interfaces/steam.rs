use std::ffi::{c_void, CStr};
use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct SteamAPIContext
{
    steam_client: *const c_void,
    steam_user: *const c_void,
    steam_friends: *const c_void,
    steam_utils: *const c_void,
    steam_matchmaking: *const c_void,
    steam_game_search: *const c_void,
    steam_user_stats: *const c_void,
    steam_apps: *const c_void,
    steam_matchmaking_servers: *const c_void,
    steam_networking: *const c_void,
    steam_remote_storage: *const c_void,
    steam_screenshots: *const c_void,
    steam_http: *const c_void,
    steam_controller: *const c_void,
    steam_ugc: *const c_void,
    steam_app_list: *const c_void,
    steam_music: *const c_void,
    steam_music_remote: *const c_void,
    pub steam_html_surface: *const SteamHTMLSurface,
    steam_inventory: *const c_void,
    steam_video: *const c_void,
    steam_parental_settings: *const c_void,
    steam_input: *const c_void,
}

#[repr(C)]
#[allow(dead_code)]
#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamHTMLSurface{}

impl SteamHTMLSurface
{
    #[virtual_index(1)]
    pub fn init(&self) -> bool {}
    
    #[virtual_index(2)]
    pub fn shutdown(&self) -> bool {}
    
    #[virtual_index(3)]
    pub fn create_browser(&self, user_agent: &CStr, user_css: &CStr) -> usize {}
    
    #[virtual_index(4)]
    pub fn remove_browser(&self, browser_handle: usize) {}
    
    #[virtual_index(5)]
    pub fn load_url(&self, browser_handle: usize, url: &CStr, post_data: &CStr) {}
    
    #[virtual_index(6)]
    pub fn set_size(&self, browser_handle: usize, width: usize, height: usize) {}
    
    #[virtual_index(7)]
    pub fn stop_load(&self, browser_handle: usize) {}
    
    #[virtual_index(8)]
    pub fn reload(&self, browser_handle: usize) {}
    
    #[virtual_index(9)]
    pub fn go_back(&self, browser_handle: usize) {}
    
    #[virtual_index(10)]
    pub fn go_forward(&self, browser_handle: usize) {}
    
    #[virtual_index(11)]
    pub fn add_header(&self, browser_handle: usize, key: &CStr, value: &CStr) {}
    
    #[virtual_index(12)]
    pub fn execute_js(&self, browser_handle: usize, js: &CStr) {}
    
    #[virtual_index(13)]
    pub fn mouse_up(&self, browser_handle: usize, mouse_button: usize) {}
}
