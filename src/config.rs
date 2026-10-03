use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub pos_x: Option<i32>,
    pub pos_y: i32,
    pub theme: String,
    pub time_format_24h: bool,
    pub show_seconds: bool,
    pub show_date: bool,
    pub size_scale: String,
    pub opacity: f32,
    pub locked: bool,
    pub always_on_top: bool,
    pub click_through: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pos_x: None,
            pos_y: 14,
            theme: "glass_dark".to_string(),
            time_format_24h: false,
            show_seconds: true,
            show_date: true,
            size_scale: "normal".to_string(),
            opacity: 0.92,
            locked: false,
            always_on_top: true,
            click_through: false,
        }
    }
}

pub fn get_config_path() -> PathBuf {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            return parent.join("config.json");
        }
    }
    PathBuf::from("config.json")
}

pub fn load_config() -> Config {
    let path = get_config_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str::<Config>(&content) {
                return config;
            }
        }
    }
    let default_config = Config::default();
    save_config(&default_config);
    default_config
}

pub fn save_config(config: &Config) {
    let path = get_config_path();
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = fs::write(path, json);
    }
}
