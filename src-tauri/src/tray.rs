use crate::tauri_windows::chat::{chat_windows, CHAT_WINDOWS};
use crate::tauri_windows::chatgpt::{show_quick_answer_window, CHATGPT_WINDOWS};
use crate::tauri_windows::search::{search_windows, SEARCH_WINDOWS};
use crate::tauri_windows::settings::{build_setting_window, SETTING_WINDOWS};
use tauri::{AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
const EXIT: &str = "EXIT";

pub fn system_tray() -> SystemTray {
    let main_page = CustomMenuItem::new(CHATGPT_WINDOWS.to_string(), "首页");
    let chat_page = CustomMenuItem::new(CHAT_WINDOWS.to_string(), "对话");
    let search_page = CustomMenuItem::new(SEARCH_WINDOWS.to_string(), "搜索");
    let setting_page = CustomMenuItem::new(SETTING_WINDOWS.to_string(), "设置");
    let exit = CustomMenuItem::new(EXIT.to_string(), "退出");
    let tray = SystemTrayMenu::new()
        .add_item(main_page)
        .add_item(chat_page)
        .add_item(search_page)
        .add_item(setting_page)
        .add_item(exit);
    SystemTray::new().with_menu(tray)
}

pub fn handle_click_system_tray(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            tracing::info!(id = id);
            match id.as_str() {
                CHATGPT_WINDOWS => show_quick_answer_window(app_handle, None, true),
                SETTING_WINDOWS => build_setting_window(app_handle),
                CHAT_WINDOWS => chat_windows(),
                SEARCH_WINDOWS => search_windows(),
                EXIT => std::process::exit(0),
                _ => {}
            }
        }
        _ => {}
    }
}
