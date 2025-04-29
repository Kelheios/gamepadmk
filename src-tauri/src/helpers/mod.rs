use once_cell::sync::OnceCell;
use tauri::{AppHandle, Manager, Wry};

#[derive(Default)]
pub struct AppState {
    pub keyboard_mode_active_key: String,
    pub keyboard_mode_active_key_held: String,
    pub keyboard_mode_multi_keys_held: std::collections::HashSet<String>,
    pub keyboard_mode_multi_keys_held_active: String,
}

static GLOBAL_APP_HANDLE: OnceCell<AppHandle<Wry>> = OnceCell::new();

pub fn app_handle_ready() -> bool {
    GLOBAL_APP_HANDLE.get().is_some()
}

pub fn get_app_handle() -> &'static AppHandle<Wry> {
    GLOBAL_APP_HANDLE.get().expect("Failed to get app handle")
}

pub fn set_app_handle(app_handle: AppHandle<Wry>) {
    GLOBAL_APP_HANDLE
        .set(app_handle)
        .expect("Failed to set app handle");
}

pub fn window_exists(label: &str) -> bool {
    get_app_handle().get_webview_window(label).is_some()
}

pub fn hide_window(label: &str) {
    let _ = get_app_handle().get_webview_window(label).unwrap().hide();
}
