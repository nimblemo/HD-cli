/// Маппинг HD Wheel: 64 ворот расположены по зодиакальному кругу (360°).
/// Каждые ворота = 5°37'30" = 5.625°
/// Каждая линия = 56'15" = 0.9375°
///
/// Порядок начинается с Ворот 41 при 2°00' Водолея (= 302° эклиптики)
/// и идёт по часовой стрелке через все 64 ворот.

/// Порядок 64 ворот по HD Wheel (начиная с Gate 41)
pub const GATE_ORDER: [u8; 64] = [
    41, 19, 13, 49, 30, 55, 37, 63,
    22, 36, 25, 17, 21, 51, 42, 3,
    27, 24, 2, 23, 8, 20, 16, 35,
    45, 12, 15, 52, 39, 53, 62, 56,
    31, 33, 7, 4, 29, 59, 40, 64,
    47, 6, 46, 18, 48, 57, 32, 50,
    28, 44, 1, 43, 14, 34, 9, 5,
    26, 11, 10, 58, 38, 54, 61, 60,
];

/// Начальный градус HD Wheel (Ворота 41 начинаются при 302.0° эклиптики)
pub const WHEEL_START_DEGREE: f64 = 302.0;

/// Размер одних ворот в градусах (5°37'30")
pub const GATE_SIZE_DEG: f64 = 5.625;

/// Размер одной линии в градусах (56'15")
pub const LINE_SIZE_DEG: f64 = 0.9375;

/// Размер одного цвета в градусах (линия / 6)
pub const COLOR_SIZE_DEG: f64 = 0.15625;

/// Результат конвертации градуса эклиптики в позицию HD
#[derive(Debug, Clone)]
pub struct GatePosition {
    pub gate: u8,
    pub line: u8,
    pub color: u8,
    pub tone: u8,
    pub base: u8,
    pub degree: f64,
}

/// Конвертация эклиптического градуса в ворота/линию/цвет/тон/базу
pub fn degree_to_gate(ecliptic_deg: f64) -> GatePosition {
    // Нормализуем градус в 0..360
    let mut deg = ecliptic_deg % 360.0;
    if deg < 0.0 {
        deg += 360.0;
    }

    // Смещение от начала колеса
    let mut offset = deg - WHEEL_START_DEGREE;
    if offset < 0.0 {
        offset += 360.0;
    }

    // Индекс ворот (0..63)
    let gate_index = (offset / GATE_SIZE_DEG).floor() as usize;
    let gate_index = gate_index.min(63);

    // Смещение внутри ворот
    let within_gate = offset - (gate_index as f64) * GATE_SIZE_DEG;

    // Линия (1..6)
    let line_index = (within_gate / LINE_SIZE_DEG).floor() as u8;
    let line = (line_index + 1).min(6);

    // Смещение внутри линии
    let within_line = within_gate - (line_index as f64) * LINE_SIZE_DEG;

    // Цвет (1..6)
    let color_index = (within_line / COLOR_SIZE_DEG).floor() as u8;
    let color = (color_index + 1).min(6);

    // Смещение внутри цвета
    let within_color = within_line - (color_index as f64) * COLOR_SIZE_DEG;

    // Тон (1..6) — каждый тон = color_size / 6
    let tone_size = COLOR_SIZE_DEG / 6.0;
    let tone_index = (within_color / tone_size).floor() as u8;
    let tone = (tone_index + 1).min(6);

    // База (1..5) — каждая база = tone_size / 5
    let within_tone = within_color - (tone_index as f64) * tone_size;
    let base_size = tone_size / 5.0;
    let base_index = (within_tone / base_size).floor() as u8;
    let base = (base_index + 1).min(5);

    GatePosition {
        gate: GATE_ORDER[gate_index],
        line,
        color,
        tone,
        base,
        degree: deg,
    }
}

/// Названия знаков зодиака
pub const ZODIAC_SIGNS: [&str; 12] = [
    "Овен", "Телец", "Близнецы", "Рак", "Лев", "Дева",
    "Весы", "Скорпион", "Стрелец", "Козерог", "Водолей", "Рыбы",
];

/// Получить знак зодиака и градус внутри знака
pub fn degree_to_zodiac(deg: f64) -> (String, f64) {
    let mut d = deg % 360.0;
    if d < 0.0 {
        d += 360.0;
    }
    let sign_index = (d / 30.0).floor() as usize;
    let within = d - (sign_index as f64) * 30.0;
    (ZODIAC_SIGNS[sign_index % 12].to_string(), within)
}
