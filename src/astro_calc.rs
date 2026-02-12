/// Астрономический движок: расчёт позиций планет через astro-rust

use astro::*;

/// Конвертация u8 месяца в time::Month
fn month_from_u8(m: u8) -> time::Month {
    match m {
        1 => time::Month::Jan,
        2 => time::Month::Feb,
        3 => time::Month::Mar,
        4 => time::Month::Apr,
        5 => time::Month::May,
        6 => time::Month::June,
        7 => time::Month::July,
        8 => time::Month::Aug,
        9 => time::Month::Sept,
        10 => time::Month::Oct,
        11 => time::Month::Nov,
        12 => time::Month::Dec,
        _ => panic!("Invalid month: {}", m),
    }
}

/// Названия планет HD
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HdPlanet {
    Sun,
    Earth,
    Moon,
    NorthNode,
    SouthNode,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
}

impl HdPlanet {
    pub fn name_ru(&self) -> &'static str {
        match self {
            HdPlanet::Sun => "Солнце",
            HdPlanet::Earth => "Земля",
            HdPlanet::Moon => "Луна",
            HdPlanet::NorthNode => "Северный Узел",
            HdPlanet::SouthNode => "Южный Узел",
            HdPlanet::Mercury => "Меркурий",
            HdPlanet::Venus => "Венера",
            HdPlanet::Mars => "Марс",
            HdPlanet::Jupiter => "Юпитер",
            HdPlanet::Saturn => "Сатурн",
            HdPlanet::Uranus => "Уран",
            HdPlanet::Neptune => "Нептун",
            HdPlanet::Pluto => "Плутон",
        }
    }

    /// Все планеты в порядке HD
    #[allow(dead_code)]
    pub fn all() -> Vec<HdPlanet> {
        vec![
            HdPlanet::Sun,
            HdPlanet::Earth,
            HdPlanet::Moon,
            HdPlanet::NorthNode,
            HdPlanet::SouthNode,
            HdPlanet::Mercury,
            HdPlanet::Venus,
            HdPlanet::Mars,
            HdPlanet::Jupiter,
            HdPlanet::Saturn,
            HdPlanet::Uranus,
            HdPlanet::Neptune,
            HdPlanet::Pluto,
        ]
    }
}

/// Расчёт Julian Day из даты, времени и UTC-смещения
pub fn calc_julian_day(year: i32, month: u8, day: u8, hour: u8, min: u8, utc_offset: f64) -> f64 {
    // Переводим в UTC
    let total_hours = hour as f64 + min as f64 / 60.0 - utc_offset;

    let day_of_month = time::DayOfMonth {
        day: day,
        hr: total_hours.floor() as u8,
        min: ((total_hours.fract().abs()) * 60.0).floor() as u8,
        sec: 0.0,
        time_zone: 0.0,
    };

    // Корректировка дня при переходе через полночь
    let mut adj_year = year;
    let mut adj_month = month;


    if total_hours < 0.0 {
        // Предыдущий день
        let (y, m, d) = prev_day(year, month, day);
        adj_year = y;
        adj_month = m;
        let adj_day = d;

        let corrected_hours = total_hours + 24.0;
        let day_of_month = time::DayOfMonth {
            day: adj_day,
            hr: corrected_hours.floor() as u8,
            min: ((corrected_hours.fract()) * 60.0).floor() as u8,
            sec: 0.0,
            time_zone: 0.0,
        };

        let date = time::Date {
            year: adj_year as i16,
            month: month_from_u8(adj_month),
            decimal_day: time::decimal_day(&day_of_month),
            cal_type: time::CalType::Gregorian,
        };
        return time::julian_day(&date);
    }

    if total_hours >= 24.0 {
        let (y, m, d) = next_day(year, month, day);
        adj_year = y;
        adj_month = m;
        let adj_day = d;

        let corrected_hours = total_hours - 24.0;
        let day_of_month = time::DayOfMonth {
            day: adj_day,
            hr: corrected_hours.floor() as u8,
            min: ((corrected_hours.fract()) * 60.0).floor() as u8,
            sec: 0.0,
            time_zone: 0.0,
        };

        let date = time::Date {
            year: adj_year as i16,
            month: month_from_u8(adj_month),
            decimal_day: time::decimal_day(&day_of_month),
            cal_type: time::CalType::Gregorian,
        };
        return time::julian_day(&date);
    }

    let date = time::Date {
        year: adj_year as i16,
        month: month_from_u8(adj_month),
        decimal_day: time::decimal_day(&day_of_month),
        cal_type: time::CalType::Gregorian,
    };
    time::julian_day(&date)
}

fn prev_day(year: i32, month: u8, day: u8) -> (i32, u8, u8) {
    if day > 1 {
        (year, month, day - 1)
    } else if month > 1 {
        let prev_month = month - 1;
        let days = days_in_month(year, prev_month);
        (year, prev_month, days)
    } else {
        (year - 1, 12, 31)
    }
}

fn next_day(year: i32, month: u8, day: u8) -> (i32, u8, u8) {
    let max = days_in_month(year, month);
    if day < max {
        (year, month, day + 1)
    } else if month < 12 {
        (year, month + 1, 1)
    } else {
        (year + 1, 1, 1)
    }
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Результат расчёта позиции планеты
#[derive(Debug, Clone)]
pub struct PlanetCalcResult {
    pub planet: HdPlanet,
    pub ecliptic_lng: f64, // в градусах
}

/// Расчёт позиций всех планет для заданного Julian Day
pub fn calc_planet_positions(jd: f64) -> Vec<PlanetCalcResult> {
    let mut results = Vec::new();

    // Земля (гелиоцентрическая, нужна для пересчёта)
    let (earth_l, earth_b, earth_r) = planet::heliocent_coords(&planet::Planet::Earth, jd);

    // Солнце (геоцентрическое)
    let (sun_ecl, _rad_vec) = sun::geocent_ecl_pos(jd);
    let sun_lng = sun_ecl.long.to_degrees();
    let sun_lng = normalize_deg(sun_lng);

    results.push(PlanetCalcResult { planet: HdPlanet::Sun, ecliptic_lng: sun_lng });

    // Земля = Солнце + 180°
    let earth_lng = normalize_deg(sun_lng + 180.0);
    results.push(PlanetCalcResult { planet: HdPlanet::Earth, ecliptic_lng: earth_lng });

    // Луна (геоцентрическая)
    let (moon_ecl, _) = lunar::geocent_ecl_pos(jd);
    let moon_lng = normalize_deg(moon_ecl.long.to_degrees());
    results.push(PlanetCalcResult { planet: HdPlanet::Moon, ecliptic_lng: moon_lng });

    // Лунные узлы (средние)
    let jc = time::julian_cent(jd);
    let mn_asc_node = lunar::mn_ascend_node(jc);
    let nn_lng = normalize_deg(mn_asc_node.to_degrees());
    let sn_lng = normalize_deg(nn_lng + 180.0);
    results.push(PlanetCalcResult { planet: HdPlanet::NorthNode, ecliptic_lng: nn_lng });
    results.push(PlanetCalcResult { planet: HdPlanet::SouthNode, ecliptic_lng: sn_lng });

    // Внутренние и внешние планеты
    let planets_list = vec![
        (HdPlanet::Mercury, planet::Planet::Mercury),
        (HdPlanet::Venus, planet::Planet::Venus),
        (HdPlanet::Mars, planet::Planet::Mars),
        (HdPlanet::Jupiter, planet::Planet::Jupiter),
        (HdPlanet::Saturn, planet::Planet::Saturn),
        (HdPlanet::Uranus, planet::Planet::Uranus),
        (HdPlanet::Neptune, planet::Planet::Neptune),
    ];

    for (hd_planet, astro_planet) in &planets_list {
        let (p_l, p_b, p_r) = planet::heliocent_coords(astro_planet, jd);
        // Геоцентрические эклиптические координаты
        let (ecl_lng, _ecl_lat, _dist, _lt) =
            planet::geocent_geomet_ecl_coords(earth_l, earth_b, earth_r, p_l, p_b, p_r);
        let lng = normalize_deg(ecl_lng.to_degrees());
        results.push(PlanetCalcResult { planet: *hd_planet, ecliptic_lng: lng });
    }

    // Плутон
    let (pluto_l, pluto_b, pluto_r) = pluto::heliocent_pos(jd);
    let (pluto_ecl_lng, _pluto_ecl_lat, _pluto_dist, _pluto_lt) =
        planet::geocent_geomet_ecl_coords(earth_l, earth_b, earth_r, pluto_l, pluto_b, pluto_r);
    let pluto_lng = normalize_deg(pluto_ecl_lng.to_degrees());
    results.push(PlanetCalcResult { planet: HdPlanet::Pluto, ecliptic_lng: pluto_lng });

    results
}

fn normalize_deg(deg: f64) -> f64 {
    let mut d = deg % 360.0;
    if d < 0.0 {
        d += 360.0;
    }
    d
}

/// Найти Julian Day когда Солнце было на 88° раньше (Design calculation)
/// Используем метод итеративного поиска
pub fn find_design_jd(birth_jd: f64, birth_sun_lng: f64) -> f64 {
    // Целевой градус Солнца = birth_sun - 88°
    let target = normalize_deg(birth_sun_lng - 88.0);

    // Примерная скорость Солнца ~0.9856°/день
    // 88° ≈ 89.3 дня назад
    let mut jd = birth_jd - 89.3;

    // Итеративный поиск (метод Ньютона-подобный)
    for _ in 0..50 {
        let (sun_ecl, _) = sun::geocent_ecl_pos(jd);
        let current_lng = normalize_deg(sun_ecl.long.to_degrees());

        let mut diff = target - current_lng;
        // Обработка перехода через 0°/360°
        if diff > 180.0 {
            diff -= 360.0;
        }
        if diff < -180.0 {
            diff += 360.0;
        }

        if diff.abs() < 0.0001 {
            break;
        }

        // Коррекция: Солнце движется ~0.9856°/день
        jd += diff / 0.9856;
    }

    jd
}
