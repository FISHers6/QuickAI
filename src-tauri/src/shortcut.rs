use crate::tauri_windows::{chatgpt, search};
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

        let easy_thing = "CommandOrControl+Q";
        handle
            .global_shortcut_manager()
            .register(easy_thing, search::search_windows)?;

        Ok(())
    }
}
