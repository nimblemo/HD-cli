use serde::Deserialize;
use std::collections::HashMap;

/// Gate structure
#[derive(Debug, Deserialize, Clone)]
pub struct GateData {
    pub name: String,
    pub description: String,
    pub lines: HashMap<String, String>,
    #[serde(default)]
    pub crosses: Vec<String>,
    #[serde(default)]
    pub center: Option<String>,
    #[serde(default)]
    pub across: Option<u8>,
    #[serde(default)]
    pub fear: Option<String>,
    #[serde(default)]
    pub sexuality: Option<String>,
    #[serde(default)]
    pub love: Option<String>,
    #[serde(default)]
    pub business: Option<String>,
    #[serde(default)]
    pub circuit: Option<String>,
    #[serde(rename = "subCircuit")]
    #[serde(default)]
    pub sub_circuit: Option<String>,
}

/// Channel structure
#[derive(Debug, Deserialize, Clone)]
pub struct ChannelData {
    #[serde(default)]
    pub name: Option<String>,
    pub description: String,
    #[serde(default)]
    pub circuit: Option<String>,
    #[serde(rename = "subCircuit")]
    #[serde(default)]
    pub sub_circuit: Option<String>,
}

/// Meta-object (for types, profiles, etc.)
#[derive(Debug, Deserialize, Clone)]
pub struct MetaObject {
    pub name: String,
    pub description: String,
}

/// Center data from DB
#[derive(Debug, Deserialize, Clone)]
pub struct CenterData {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub normal: String,
    pub distorted: String,
}



/// PHS Block (Colors/Tones)
#[derive(Debug, Deserialize, Clone)]
pub struct PhsBlock {
    #[serde(default)]
    pub colors: HashMap<String, String>,
    #[serde(default)]
    pub tones: HashMap<String, String>,
}

/// Main database structure
#[derive(Debug, Deserialize)]
pub struct HdDatabase {
    pub gates: HashMap<String, GateData>,
    pub channels: HashMap<String, ChannelData>,
    pub centers: HashMap<String, CenterData>,
    pub types: HashMap<String, MetaObject>,
    pub profiles: HashMap<String, MetaObject>,
    #[serde(default)]
    pub strategies: HashMap<String, String>, 
    pub authorities: HashMap<String, MetaObject>,
    #[serde(default)]
    pub fears: HashMap<String, String>,
    #[serde(default)]
    pub motivation: Option<PhsBlock>,
    #[serde(default)]
    pub environment: Option<PhsBlock>,
    #[serde(default)]
    pub diet: Option<PhsBlock>,
    #[serde(default)]
    pub vision: Option<PhsBlock>,
    #[serde(default)]
    pub crosses: HashMap<String, MetaObject>,
}

use once_cell::sync::Lazy;

// Path relative to src/data/database.rs -> ../../data/gates_database.json
const DEFAULT_DB_JSON: &str = include_str!("../../data/gates_database.json");

static DEFAULT_DB: Lazy<HdDatabase> = Lazy::new(|| {
    serde_json::from_str(DEFAULT_DB_JSON).unwrap_or_else(|e| {
        panic!("Failed to parse embedded gates_database.json: {}", e)
    })
});

/// Get database (embedded or loaded by language)
pub fn get_database(lang: &str) -> &'static HdDatabase {
    if lang == "ru" {
        &DEFAULT_DB
    } else {
        // For other languages return default for now
        eprintln!("Язык '{}' не найден, используется 'ru'", lang);
        &DEFAULT_DB
    }
}
