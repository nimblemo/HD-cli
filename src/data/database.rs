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

// Embed all three databases
const DB_JSON_RU: &str = include_str!("../../data/gates_database_ru.json");
const DB_JSON_EN: &str = include_str!("../../data/gates_database_en.json");
const DB_JSON_ES: &str = include_str!("../../data/gates_database_es.json");

static DB_RU: Lazy<HdDatabase> = Lazy::new(|| {
    serde_json::from_str(DB_JSON_RU).expect("Failed to parse embedded gates_database_ru.json")
});
static DB_EN: Lazy<HdDatabase> = Lazy::new(|| {
    serde_json::from_str(DB_JSON_EN).expect("Failed to parse embedded gates_database_en.json")
});
static DB_ES: Lazy<HdDatabase> = Lazy::new(|| {
    serde_json::from_str(DB_JSON_ES).expect("Failed to parse embedded gates_database_es.json")
});

/// Get database by language code
pub fn get_database(lang: &str) -> &'static HdDatabase {
    match lang {
        "en" => &DB_EN,
        "es" => &DB_ES,
        _ => &DB_RU, // Default to RU
    }
}
