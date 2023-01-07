use std::ffi::c_void;

#[repr(C)]
#[allow(dead_code)]
struct SteamAPIContext
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
    steam_html_surface: *const c_void,
    steam_inventory: *const c_void,
    steam_video: *const c_void,
    steam_parental_settings: *const c_void,
    steam_input: *const c_void,
}
