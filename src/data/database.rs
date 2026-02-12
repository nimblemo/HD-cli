use serde::Deserialize;
use std::collections::HashMap;

/// Структура ворот (Gate)
#[derive(Debug, Deserialize, Clone)]
pub struct GateData {
    #[allow(dead_code)]
    pub id: String,
    pub name: String,
    pub description: String,
    pub lines: HashMap<String, String>,
    #[serde(default)]
    pub crosses: Vec<String>,
    #[serde(default)]
    pub business: Option<BusinessData>,
    #[serde(default)]
    pub sexuality: Option<SexualityData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BusinessData {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SexualityData {
    pub title: String,
    pub description: String,
}

/// Структура канала
#[derive(Debug, Deserialize, Clone)]
pub struct ChannelData {
    pub name: String,
    pub description: String,
}

/// Мотивация / Среда / Диета - с цветами и тонами
#[derive(Debug, Deserialize, Clone)]
pub struct ColorToneData {
    #[serde(default)]
    pub colors: HashMap<String, String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub tones: HashMap<String, String>,
}

/// Данные тона диеты
#[derive(Debug, Deserialize, Clone)]
pub struct ToneData {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub description: String,
}

/// Диета
#[derive(Debug, Deserialize, Clone)]
pub struct DietData {
    #[serde(default)]
    pub colors: HashMap<String, String>,
    #[serde(default)]
    pub tones: HashMap<String, ToneData>,
}

/// Описание центра
#[derive(Debug, Deserialize, Clone)]
pub struct CenterBehavior {
    #[serde(rename = "normalBehavior")]
    pub normal_behavior: String,
    #[serde(rename = "distortedBehavior")]
    pub distorted_behavior: String,
}

/// Главная структура базы данных
#[derive(Debug, Deserialize)]
pub struct HdDatabase {
    pub gates: HashMap<String, GateData>,
    pub channels: HashMap<String, ChannelData>,
    pub profiles: HashMap<String, String>,
    pub types: HashMap<String, String>,
    #[serde(default)]
    pub strategies: HashMap<String, String>,
    #[serde(default)]
    pub authorities: HashMap<String, String>,
    #[serde(default)]
    pub centers: HashMap<String, CenterBehavior>,
    #[serde(default)]
    pub fears: HashMap<String, String>,
    #[serde(default)]
    pub motivation: Option<ColorToneData>,
    #[serde(default)]
    pub environment: Option<ColorToneData>,
    #[serde(default)]
    pub diet: Option<DietData>,
    #[serde(default)]
    pub vision: Option<ColorToneData>,
    #[serde(default)]
    pub crosses: HashMap<String, String>,
}

use once_cell::sync::Lazy;

const DEFAULT_DB_JSON: &str = include_str!("../../data/gates_database.json");

static DEFAULT_DB: Lazy<HdDatabase> = Lazy::new(|| {
    serde_json::from_str(DEFAULT_DB_JSON).unwrap_or_else(|e| {
        panic!("Failed to parse embedded gates_database.json: {}", e)
    })
});

/// Получить базу данных (встроенную или загруженную по языку)
pub fn get_database(lang: &str) -> &'static HdDatabase {
    if lang == "ru" {
        &DEFAULT_DB
    } else {
        // Для других языков пока возвращаем дефолтную
        // В будущем — загрузка data/gates_database_{lang}.json рядом с бинарником
        eprintln!("Язык '{}' не найден, используется 'ru'", lang);
        &DEFAULT_DB
    }
}
