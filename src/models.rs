use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CircuitScoreItem {
    pub circuit: String,
    pub circuit_name: String,
    pub circuit_description: String,
    pub sub_circuit: String,
    pub sub_circuit_name: String,
    pub score: f64,
    pub planet_count: usize,
    pub channel_count: usize,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlanetPosition {
    pub planet: String,
    pub index: usize,
    pub longitude: f64,
    pub degree: f64, // 0..360
    pub zodiac_sign: String,
    pub zodiac_symbol: String, // e.g. "♉"
    pub planet_symbol: String, // e.g. "☉"
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

#[derive(Debug, Clone, Serialize)]
pub struct ChannelInfo {
    pub key: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CenterInfo {
    pub name: String,
    pub defined: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_normal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_distorted: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HdChart {
    pub birth_date: String,
    pub birth_time: String,
    pub utc_offset: f64,

    #[serde(rename = "type")]
    pub hd_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    pub profile: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_description: Option<String>,
    pub authority: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_description: Option<String>,
    pub strategy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_description: Option<String>,
    pub incarnation_cross: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_description: Option<String>,
    pub personality: Vec<PlanetPosition>,
    pub design: Vec<PlanetPosition>,
    pub channels: Vec<ChannelInfo>,
    pub centers: Vec<CenterInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<Vec<InfoItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivation: Option<Vec<InfoItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<InfoItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diet: Option<Vec<InfoItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fear: Option<Vec<InfoItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexuality: Option<Vec<InfoItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub love: Option<Vec<InfoItem>>,
    /// Perspective / Vision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vision: Option<Vec<InfoItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_scores: Option<Vec<CircuitScoreItem>>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlanetShortInfo {
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct InfoItem {
    pub label: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planets: Option<std::collections::HashSet<PlanetShortInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gate_id: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gate_name: Option<String>,
}
