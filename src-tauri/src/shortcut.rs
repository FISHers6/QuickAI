
use tauri::AppHandle;
use tauri::GlobalShortcutManager;
use anyhow::Result;
use crate::windows;

pub(crate) struct ShortcutRegister;

impl ShortcutRegister {
    pub fn register_shortcut(handle: &AppHandle) -> Result<()> {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        handle.global_shortcut_manager().unregister_all()?;
        // 依次注册快捷键
        let shortcut = "CommandOrControl+D";
        handle
        .global_shortcut_manager()
        .register(shortcut, windows::chatgpt_windows)?;
        Ok(())
    }
}
