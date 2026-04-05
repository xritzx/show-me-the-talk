use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const SETTINGS_FILENAME: &str = "settings.json";
const APP_DIR_NAME: &str = "show-me-the-talk";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub llm_enabled: bool,
    pub include_sql_instructions: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            llm_enabled: false,
            include_sql_instructions: false,
        }
    }
}

fn settings_path() -> Result<PathBuf, String> {
    let data_dir = dirs::data_dir().ok_or("Could not determine app data directory")?;
    Ok(data_dir.join(APP_DIR_NAME).join(SETTINGS_FILENAME))
}

pub fn load_settings() -> AppSettings {
    let path = match settings_path() {
        Ok(p) => p,
        Err(e) => {
            log::warn!("Settings path error, using defaults: {}", e);
            return AppSettings::default();
        }
    };
    if !path.exists() {
        return AppSettings::default();
    }
    match std::fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
            log::warn!("Malformed settings file, using defaults: {}", e);
            AppSettings::default()
        }),
        Err(e) => {
            log::warn!("Failed to read settings, using defaults: {}", e);
            AppSettings::default()
        }
    }
}

pub fn save_settings(settings: &AppSettings) -> Result<(), String> {
    let path = settings_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;
    }
    let json =
        serde_json::to_string_pretty(settings).map_err(|e| format!("Serialize error: {}", e))?;
    std::fs::write(&path, json).map_err(|e| format!("Failed to write settings: {}", e))
}
