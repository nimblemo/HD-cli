/// 36 Human Design Channels
/// Each channel: (gate_a, gate_b, center_a, center_b)
use super::centers::Center;

#[derive(Debug, Clone)]
pub struct ChannelDef {
    pub gate_a: u8,
    pub gate_b: u8,
    pub center_a: Center,
    pub center_b: Center,
}

impl ChannelDef {
    pub fn key(&self) -> String {
        if self.gate_a < self.gate_b {
            format!("{}-{}", self.gate_a, self.gate_b)
        } else {
            format!("{}-{}", self.gate_b, self.gate_a)
        }
    }
}

/// All 36 HD channels
pub fn all_channels() -> Vec<ChannelDef> {
    use Center::*;
    vec![
        // Format channels
        ChannelDef { gate_a: 64, gate_b: 47, center_a: Head, center_b: Ajna },
        ChannelDef { gate_a: 61, gate_b: 24, center_a: Head, center_b: Ajna },
        ChannelDef { gate_a: 63, gate_b: 4, center_a: Head, center_b: Ajna },
        // Ajna → Throat
        ChannelDef { gate_a: 17, gate_b: 62, center_a: Ajna, center_b: Throat },
        ChannelDef { gate_a: 43, gate_b: 23, center_a: Ajna, center_b: Throat },
        ChannelDef { gate_a: 11, gate_b: 56, center_a: Ajna, center_b: Throat },
        // Throat connections
        ChannelDef { gate_a: 7, gate_b: 31, center_a: G, center_b: Throat },
        ChannelDef { gate_a: 1, gate_b: 8, center_a: G, center_b: Throat },
        ChannelDef { gate_a: 13, gate_b: 33, center_a: G, center_b: Throat },
        ChannelDef { gate_a: 10, gate_b: 20, center_a: G, center_b: Throat },
        ChannelDef { gate_a: 16, gate_b: 48, center_a: Throat, center_b: Spleen },
        ChannelDef { gate_a: 20, gate_b: 57, center_a: Throat, center_b: Spleen },
        ChannelDef { gate_a: 20, gate_b: 34, center_a: Throat, center_b: Sacral },
        ChannelDef { gate_a: 12, gate_b: 22, center_a: Throat, center_b: SolarPlexus },
        ChannelDef { gate_a: 35, gate_b: 36, center_a: Throat, center_b: SolarPlexus },
        ChannelDef { gate_a: 45, gate_b: 21, center_a: Throat, center_b: Heart },
        // G Center
        ChannelDef { gate_a: 2, gate_b: 14, center_a: G, center_b: Sacral },
        ChannelDef { gate_a: 10, gate_b: 34, center_a: G, center_b: Sacral },
        ChannelDef { gate_a: 25, gate_b: 51, center_a: G, center_b: Heart },
        ChannelDef { gate_a: 15, gate_b: 5, center_a: G, center_b: Sacral },
        ChannelDef { gate_a: 46, gate_b: 29, center_a: G, center_b: Sacral },
        // Heart
        ChannelDef { gate_a: 26, gate_b: 44, center_a: Heart, center_b: Spleen },
        ChannelDef { gate_a: 40, gate_b: 37, center_a: Heart, center_b: SolarPlexus },
        // Sacral
        ChannelDef { gate_a: 59, gate_b: 6, center_a: Sacral, center_b: SolarPlexus },
        ChannelDef { gate_a: 27, gate_b: 50, center_a: Sacral, center_b: Spleen },
        ChannelDef { gate_a: 34, gate_b: 57, center_a: Sacral, center_b: Spleen },
        ChannelDef { gate_a: 5, gate_b: 15, center_a: Sacral, center_b: G },   // duplicate handled by key
        ChannelDef { gate_a: 14, gate_b: 2, center_a: Sacral, center_b: G },    // duplicate handled by key
        ChannelDef { gate_a: 29, gate_b: 46, center_a: Sacral, center_b: G },   // duplicate handled by key
        ChannelDef { gate_a: 42, gate_b: 53, center_a: Sacral, center_b: Root },
        ChannelDef { gate_a: 3, gate_b: 60, center_a: Sacral, center_b: Root },
        ChannelDef { gate_a: 9, gate_b: 52, center_a: Sacral, center_b: Root },
        // Spleen
        ChannelDef { gate_a: 57, gate_b: 20, center_a: Spleen, center_b: Throat },
        ChannelDef { gate_a: 18, gate_b: 58, center_a: Spleen, center_b: Root },
        ChannelDef { gate_a: 28, gate_b: 38, center_a: Spleen, center_b: Root },
        ChannelDef { gate_a: 32, gate_b: 54, center_a: Spleen, center_b: Root },
        // Solar Plexus → Root
        ChannelDef { gate_a: 39, gate_b: 55, center_a: Root, center_b: SolarPlexus },
        ChannelDef { gate_a: 41, gate_b: 30, center_a: Root, center_b: SolarPlexus },
        ChannelDef { gate_a: 19, gate_b: 49, center_a: Root, center_b: SolarPlexus },
    ]
}

/// Find active channels by set of active gates
pub fn find_active_channels(active_gates: &[u8]) -> Vec<ChannelDef> {
    all_channels()
        .into_iter()
        .filter(|ch| {
            active_gates.contains(&ch.gate_a) && active_gates.contains(&ch.gate_b)
        })
        .collect()
}

/// Remove channel duplicates (by key)
pub fn unique_channels(channels: Vec<ChannelDef>) -> Vec<ChannelDef> {
    let mut seen = std::collections::HashSet::new();
    channels
        .into_iter()
        .filter(|ch| seen.insert(ch.key()))
        .collect()
}
