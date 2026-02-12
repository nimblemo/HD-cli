/// 9 центров Human Design
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
    pub fn name_ru(&self) -> &'static str {
        match self {
            Center::Head => "Теменной",
            Center::Ajna => "Аджна",
            Center::Throat => "Горловой",
            Center::G => "Джи",
            Center::Heart => "Сердечный",
            Center::Sacral => "Сакральный",
            Center::SolarPlexus => "Солнечное сплетение",
            Center::Spleen => "Селезёночный",
            Center::Root => "Корневой",
        }
    }

    /// Все центры
    pub fn all() -> Vec<Center> {
        vec![
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

    /// Ключ центра для поиска в БД
    pub fn db_key(&self) -> &'static str {
        match self {
            Center::Head => "Теменной Центр",
            Center::Ajna => "Аджна, Центр Ума", // Was "Центр Аджна"
            Center::Throat => "Горловой центр", // Was "Горловой Центр" (case)
            Center::G => "Центр G", // Was "Джи Центр"
            Center::Heart => "Центр Эго", // Was "Сердечный Центр"
            Center::Sacral => "Сакральный Центр",
            Center::SolarPlexus => "Эмоциональный центр", // Was "Центр Солнечного Сплетения"
            Center::Spleen => "Центр Селезенки", // Was "Селезёночный Центр"
            Center::Root => "Корневой Центр",
        }
    }

    /// Является ли центр моторным
    pub fn is_motor(&self) -> bool {
        matches!(self, Center::Sacral | Center::Heart | Center::SolarPlexus | Center::Root)
    }
}

/// Ворота, принадлежащие каждому центру
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
