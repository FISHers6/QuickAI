use crate::tauri_windows::{chatgpt, search, chat};
use anyhow::Result;
use tauri::AppHandle;
use tauri::GlobalShortcutManager;

pub(crate) struct ShortcutRegister;

impl ShortcutRegister {
    pub fn register_shortcut(handle: &AppHandle) -> Result<()> {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        handle.global_shortcut_manager().unregister_all()?;
        // 依次注册快捷键
        let shortcut = "CommandOrControl+D";
        handle
            .global_shortcut_manager()
            .register(shortcut, chatgpt::chatgpt_windows)?;

        let easy_thing = "Shift+Space";
        handle
            .global_shortcut_manager()
            .register(easy_thing, search::search_windows)?;

        let chat_shortcut = "Shift+C";
        handle
            .global_shortcut_manager()
            .register(chat_shortcut, chat::chat_windows)?;

        Ok(())
    }
}
