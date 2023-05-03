pub mod chatgpt;
pub mod search;
pub mod chat;
#[cfg(not(target_os="macos"))]
pub mod select;
pub mod settings;

pub const SELECT_WINDOWS: &str = "select_windows";