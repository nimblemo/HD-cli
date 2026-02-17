/// Output models for serialization
use serde::Serialize;

/// Planet position in chart
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

/// Channel info
#[derive(Debug, Clone, Serialize)]
pub struct ChannelInfo {
    /// Channel key (e.g. "1-8")
    pub key: String,
    /// Channel name
    pub name: String,
    /// Description (with --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Center info
#[derive(Debug, Clone, Serialize)]
pub struct CenterInfo {
    /// Center name
    pub name: String,
    /// Is defined
    pub defined: bool,
    /// Behavior (with --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_normal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_distorted: Option<String>,
}

/// Main chart
#[derive(Debug, Clone, Serialize)]
pub struct HdChart {
    /// Input data
    pub birth_date: String,
    pub birth_time: String,
    pub utc_offset: f64,

    /// Type
    #[serde(rename = "type")]
    pub hd_type: String,
    /// Type description (with --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,

    /// Profile
    pub profile: String,
    /// Profile description (with --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_description: Option<String>,

    /// Authority
    pub authority: String,
    /// Authority description (with --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_description: Option<String>,

    /// Strategy
    pub strategy: String,
    /// Strategy description (with --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_description: Option<String>,

    /// Incarnation Cross
    pub incarnation_cross: String,
    /// Cross description (with --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_description: Option<String>,

    /// Planet positions (Personality)
    pub personality: Vec<PlanetPosition>,
    /// Planet positions (Design)
    pub design: Vec<PlanetPosition>,

    /// Channels
    pub channels: Vec<ChannelInfo>,

    /// Centers
    pub centers: Vec<CenterInfo>,

    /// Business (with --full)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<Vec<InfoItem>>,

    /// Motivation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivation: Option<Vec<InfoItem>>,

    /// Environment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<InfoItem>>,

    /// Diet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diet: Option<Vec<InfoItem>>,

    /// Fear
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fear: Option<Vec<InfoItem>>,

    /// Sexuality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexuality: Option<Vec<InfoItem>>,

    /// Love
    #[serde(skip_serializing_if = "Option::is_none")]
    pub love: Option<Vec<InfoItem>>,

    /// Perspective / Vision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vision: Option<Vec<InfoItem>>,
}

/// Short planet info for lists
/// Short planet info for lists
#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlanetShortInfo {
    pub name: String,
    pub symbol: String,
}

/// Additional info item (Label + Description + Optional Gate Context)
#[derive(Debug, Clone, Serialize)]
pub struct InfoItem {
    pub label: String,
    pub description: String,

    /// Optional: Planets activating this item (for Fear/Love/etc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planets: Option<std::collections::HashSet<PlanetShortInfo>>,

    /// Optional: Gate ID if related to a specific gate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gate_id: Option<u8>,

    /// Optional: Gate Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gate_name: Option<String>,
}
