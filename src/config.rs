use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub language: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: "ru".to_string(), // Default language is Russian
        }
    }
}

impl Config {
    /// Load configuration from file or return default
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        if let Some(path) = config_path {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(config) = serde_json::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::get_config_path().ok_or("Could not determine config path")?;
        
        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(config_path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Set default language
    pub fn set_language(&mut self, lang: &str) -> Result<(), String> {
        match lang {
            "en" | "ru" | "es" => {
                self.language = lang.to_string();
                self.save()
            }
            _ => Err(format!("Unsupported language: {}. Supported: en, ru, es", lang)),
        }
    }

    fn get_config_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "nimblemo", "hd-cli")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.json"))
    }
}
