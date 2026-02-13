/// HD Wheel Mapping: 64 gates located on the zodiac circle (360°).
/// Each gate = 5°37'30" = 5.625°
/// Each line = 56'15" = 0.9375°
///
/// Order starts with Gate 41 at 2°00' Aquarius (= 302° ecliptic)
/// and goes clockwise through all 64 gates.

/// Order of 64 gates on HD Wheel (starting from Gate 41)
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

/// Initial HD Wheel degree (Gate 41 starts at 302.0° ecliptic)
pub const WHEEL_START_DEGREE: f64 = 302.0;

/// Size of one gate in degrees (5°37'30")
pub const GATE_SIZE_DEG: f64 = 5.625;

/// Size of one line in degrees (56'15")
pub const LINE_SIZE_DEG: f64 = 0.9375;

/// Size of one color in degrees (line / 6)
pub const COLOR_SIZE_DEG: f64 = 0.15625;

/// Result of converting ecliptic degree to HD position
#[derive(Debug, Clone)]
pub struct GatePosition {
    pub gate: u8,
    pub line: u8,
    pub color: u8,
    pub tone: u8,
    pub base: u8,
    pub degree: f64,
}

/// Convert ecliptic degree to gate/line/color/tone/base
pub fn degree_to_gate(ecliptic_deg: f64) -> GatePosition {
    // Normalize degree to 0..360
    let mut deg = ecliptic_deg % 360.0;
    if deg < 0.0 {
        deg += 360.0;
    }

    // Offset from wheel start
    let mut offset = deg - WHEEL_START_DEGREE;
    if offset < 0.0 {
        offset += 360.0;
    }

    // Gate index (0..63)
    let gate_index = (offset / GATE_SIZE_DEG).floor() as usize;
    let gate_index = gate_index.min(63);

    // Offset within gate
    let within_gate = offset - (gate_index as f64) * GATE_SIZE_DEG;

    // Line (1..6)
    let line_index = (within_gate / LINE_SIZE_DEG).floor() as u8;
    let line = (line_index + 1).min(6);

    // Offset within line
    let within_line = within_gate - (line_index as f64) * LINE_SIZE_DEG;

    // Color (1..6)
    let color_index = (within_line / COLOR_SIZE_DEG).floor() as u8;
    let color = (color_index + 1).min(6);

    // Offset within color
    let within_color = within_line - (color_index as f64) * COLOR_SIZE_DEG;

    // Tone (1..6) — each tone = color_size / 6
    let tone_size = COLOR_SIZE_DEG / 6.0;
    let tone_index = (within_color / tone_size).floor() as u8;
    let tone = (tone_index + 1).min(6);

    // Base (1..5) — each base = tone_size / 5
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

/// Zodiac sign names
/// Zodiac sign keys
pub const ZODIAC_SIGNS: [&str; 12] = [
    "aries", "taurus", "gemini", "cancer", "leo", "virgo",
    "libra", "scorpio", "sagittarius", "capricorn", "aquarius", "pisces",
];

/// Get zodiac sign and degree within sign
pub fn degree_to_zodiac(deg: f64) -> (String, f64) {
    let mut d = deg % 360.0;
    if d < 0.0 {
        d += 360.0;
    }
    let sign_index = (d / 30.0).floor() as usize;
    let within = d - (sign_index as f64) * 30.0;
    (ZODIAC_SIGNS[sign_index % 12].to_string(), within)
}
