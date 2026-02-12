/// Выходные модели для сериализации

use serde::Serialize;

/// Позиция планеты в карте
#[derive(Debug, Clone, Serialize)]
pub struct PlanetPosition {
    pub planet: String,
    pub index: usize,
    pub longitude: f64,
    pub degree: f64,           // 0..360
    pub zodiac_sign: String,
    pub zodiac_symbol: String, // e.g. "♉"
    pub zodiac_degree: f64,    // 0..30
    pub gate: u8,
    pub line: u8,
    pub color: u8,
    pub tone: u8,
    pub base: u8,
    pub gate_name: Option<String>,
    pub gate_description: Option<String>,
    pub line_description: Option<String>,
}

/// Информация о канале
#[derive(Debug, Clone, Serialize)]
pub struct ChannelInfo {
    /// Ключ канала (e.g. "1-8")
    pub key: String,
    /// Название канала
    pub name: String,
    /// Описание (при --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Информация о центре
#[derive(Debug, Clone, Serialize)]
pub struct CenterInfo {
    /// Название центра
    pub name: String,
    /// Определён ли
    pub defined: bool,
    /// Поведение (при --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<String>,
}

/// Основная карта
#[derive(Debug, Clone, Serialize)]
pub struct HdChart {
    /// Входные данные
    pub birth_date: String,
    pub birth_time: String,
    pub utc_offset: f64,

    /// Тип
    #[serde(rename = "type")]
    pub hd_type: String,
    /// Описание типа (при --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,

    /// Профиль
    pub profile: String,
    /// Описание профиля (при --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_description: Option<String>,

    /// Авторитет
    pub authority: String,
    /// Описание авторитета (при --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_description: Option<String>,

    /// Стратегия
    pub strategy: String,
    /// Описание стратегии (при --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_description: Option<String>,

    /// Инкарнационный крест
    pub incarnation_cross: String,
    /// Описание креста (при --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_description: Option<String>,

    /// Позиции планет (Личность)
    pub personality: Vec<PlanetPosition>,
    /// Позиции планет (Дизайн)
    pub design: Vec<PlanetPosition>,

    /// Каналы
    pub channels: Vec<ChannelInfo>,

    /// Центры
    pub centers: Vec<CenterInfo>,

    /// Бизнес (при --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<Vec<BusinessInfo>>,

    /// Мотивация
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivation: Option<Vec<InfoItem>>,

    /// Среда
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<InfoItem>>,

    /// Диета
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diet: Option<Vec<InfoItem>>,

    /// Страх
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fear: Option<String>,

    /// Сексуальность
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexuality: Option<String>,

    /// Любовь
    #[serde(skip_serializing_if = "Option::is_none")]
    pub love: Option<String>,

    /// Видение (Perspective/View)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vision: Option<Vec<InfoItem>>,
}

/// Элемент дополнительной информации (Заголовок + Описание)
#[derive(Debug, Clone, Serialize)]
pub struct InfoItem {
    pub label: String,
    pub description: String,
}

/// Бизнес-информация от ворот
#[derive(Debug, Clone, Serialize)]
pub struct BusinessInfo {
    pub gate: u8,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
