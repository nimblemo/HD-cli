/// 9 Human Design Centers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Center {
    Head,
    Ajna,
    Throat,
    G,
    Heart,
    Sacral,
    SolarPlexus,
    Spleen,
    Root,
}

impl Center {
    /// Get list of all centers
    pub fn all() -> &'static [Center] {
        &[
            Center::Head,
            Center::Ajna,
            Center::Throat,
            Center::G,
            Center::Heart,
            Center::Sacral,
            Center::SolarPlexus,
            Center::Spleen,
            Center::Root,
        ]
    }

    /// Center key for DB lookup (latin keys)
    pub fn key(&self) -> &'static str {
        match self {
            Center::Head => "head",
            Center::Ajna => "ajna",
            Center::Throat => "throat",
            Center::G => "g",
            Center::Heart => "heart",
            Center::Sacral => "sacral",
            Center::SolarPlexus => "solar_plexus",
            Center::Spleen => "splenic",
            Center::Root => "root",
        }
    }

    /// Is the center a motor
    pub fn is_motor(&self) -> bool {
        matches!(self, Center::Sacral | Center::Heart | Center::SolarPlexus | Center::Root)
    }
}

/// Gates belonging to each center
#[allow(dead_code)]
pub fn gates_for_center(center: &Center) -> Vec<u8> {
    match center {
        Center::Head => vec![64, 61, 63],
        Center::Ajna => vec![47, 24, 4, 17, 43, 11],
        Center::Throat => vec![62, 23, 56, 31, 8, 33, 20, 16, 12, 35, 45],
        Center::G => vec![7, 1, 13, 10, 2, 25, 15, 46],
        Center::Heart => vec![21, 51, 26, 40],
        Center::Sacral => vec![14, 34, 5, 29, 59, 27, 42, 3, 9],
        Center::SolarPlexus => vec![22, 36, 6, 37, 55, 30, 49],
        Center::Spleen => vec![48, 57, 44, 50, 32, 28, 18],
        Center::Root => vec![53, 60, 52, 19, 39, 41, 58, 38, 54],
    }
}
