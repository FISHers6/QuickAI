pub mod chat;
pub mod chatgpt;
pub mod search;
#[cfg(not(target_os = "macos"))]
pub mod select;
pub mod settings;

pub const SELECT_WINDOWS: &str = "select_windows";
