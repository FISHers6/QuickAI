use tauri::api::path::config_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub quick_ask_shortcut: Option<String>,
    pub search_shortcut: Option<String>,
    pub chat_shortcut: Option<String>,
    pub mode: Option<String>,
    pub is_dark_mode: bool,
    pub language: String,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub use_chat_context: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { 
            quick_ask_shortcut: Some("CommandOrControl+Alt+D".to_string()), 
            search_shortcut: Some("Shift+Alt+Space".to_string()), 
            chat_shortcut: Some("Shift+Alt+C".to_string()),
            mode: Some("快捷提问".to_string()),
            is_dark_mode: true,
            language: "zh-cn".to_string(),
            api_key: None,
            proxy: None,
            use_chat_context: true, 
        }
    }
}

#[tauri::command]
pub fn get_app_config_json() -> String {
    if let Some(config_dir) = config_dir() {
        let app_config_dir = config_dir.join("config.desk-ai");
        if !app_config_dir.exists() {
            std::fs::create_dir_all(&app_config_dir).expect("not failed");
        }
        let config_path = app_config_dir.join("config.json");
        if config_path.exists() {
            match std::fs::read_to_string(config_path) {
                Ok(content) => content,
                Err(_) => serde_json::to_string(&AppConfig::default()).expect("not failed"),
            }
        } else {
            if let Ok(config_str) = serde_json::to_string(&AppConfig::default()) {
                let _  = std::fs::write(config_path, config_str.clone());
                config_str
            }else {
                serde_json::to_string(&AppConfig::default()).expect("not failed")
            }
        }
    } else {
        serde_json::to_string(&AppConfig::default()).expect("not failed")
    }
}


pub fn get_app_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_content = get_app_config_json();
    let config: AppConfig = serde_json::from_str(&config_content)?;
    Ok(config)
}


pub fn save_app_config(config: &AppConfig) -> Result<(), String> {
    if let Some(config_dir) = config_dir() {
        let app_config_dir = config_dir.join("config.desk-ai");
        if !app_config_dir.exists() {
            std::fs::create_dir_all(&app_config_dir).expect("not failed");
        }
        let config_path = app_config_dir.join("config.json");
        if let Ok(config_str) = serde_json::to_string(config) {
            std::fs::write(config_path, config_str).map_err(|err| {
                format!("failed to write config {}", err)
            })
        }else {
            Err("serialize app config error".to_string())
        }
    }else {
        Err("not found app config directory".to_string())
    }
}

