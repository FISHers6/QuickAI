use crate::tauri_windows::{chatgpt, search, chat};
use anyhow::Result;
use tauri::AppHandle;
use tauri::GlobalShortcutManager;

pub(crate) struct ShortcutRegister;

impl ShortcutRegister {
    pub fn register_shortcut(handle: &AppHandle) -> Result<()> {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        handle.global_shortcut_manager().unregister_all()?;

        let app_config = crate::app_config::get_app_config().unwrap_or_default();
        // 依次注册快捷键
        let shortcut = app_config.quick_ask_shortcut.unwrap_or_default();
        if !shortcut.is_empty() {
            handle
            .global_shortcut_manager()
            .register(&shortcut, chatgpt::chatgpt_shortcut)?;
        }

        let easy_thing = app_config.search_shortcut.unwrap_or_default();
        if !easy_thing.is_empty() {
            handle
            .global_shortcut_manager()
            .register(&easy_thing, search::search_windows)?;
        }

        let chat_shortcut = app_config.chat_shortcut.unwrap_or_default();
        if !chat_shortcut.is_empty() {
            handle
            .global_shortcut_manager()
            .register(&chat_shortcut, chat::chat_windows)?;
        }

        Ok(())
    }
}