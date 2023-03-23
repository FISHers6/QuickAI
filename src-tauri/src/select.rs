use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use anyhow::{Result, anyhow};

#[cfg(target_os = "windows")]
pub fn copy() {
    use enigo::*;
    let mut enigo = Enigo::new();
    enigo.key_up(Key::Control);
    enigo.key_up(Key::Alt);
    enigo.key_up(Key::Shift);
    enigo.key_up(Key::Space);
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('c'));
    enigo.key_up(Key::Control);
}

#[cfg(target_os = "macos")]
pub fn copy() {
    use enigo::*;
    let mut enigo = Enigo::new();
    enigo.key_up(Key::Control);
    enigo.key_up(Key::Meta);
    enigo.key_up(Key::Alt);
    enigo.key_up(Key::Shift);
    enigo.key_up(Key::Space);
    enigo.key_up(Key::Tab);
    enigo.key_up(Key::Option);
    enigo.key_down(Key::Meta);
    enigo.key_click(Key::Layout('c'));
    enigo.key_up(Key::Meta);
}

#[cfg(target_os = "linux")]
pub fn copy() {
    use enigo::*;
    let mut enigo = Enigo::new();
    enigo.key_up(Key::Control);
    enigo.key_up(Key::Alt);
    enigo.key_up(Key::Shift);
    enigo.key_up(Key::Space);
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('c'));
    enigo.key_up(Key::Control);
}

pub fn selected_text() -> Result<String> {
    let mut cli_pboard: ClipboardContext = ClipboardProvider::new().map_err(|_err| {
        anyhow!("get clipboard error")
    })?;
    let old_text = cli_pboard.get_contents().map_err(|_err| {
        anyhow!("get clipboard content error")
    })?;
    copy();
    if let Ok(new_text) = cli_pboard.get_contents() {
        let _err = cli_pboard.set_contents(old_text);
        Ok(new_text)
    }else {
        let _err = cli_pboard.set_contents(old_text.clone());
        Ok(old_text)
    }
}
