/// HD расчёты: тип, профиль, авторитет, каналы, центры, крест

use crate::astro_calc::{self, HdPlanet};
use crate::data::centers::Center;
use crate::data::channels::{self, ChannelDef};
use crate::data::database::{self, HdDatabase};
use crate::data::gates;
use crate::models::*;
use std::collections::HashSet;

/// Построить полную карту
pub fn build_chart(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    min: u8,
    utc_offset: f64,
    full: bool,
    lang: &str,
) -> HdChart {
    let db = database::get_database(lang);

    // 1. Julian Day для Личности (момент рождения)
    let personality_jd = astro_calc::calc_julian_day(year, month, day, hour, min, utc_offset);

    // 2. Позиции планет Личности
    let personality_positions = astro_calc::calc_planet_positions(personality_jd);

    // 3. Найти Julian Day для Дизайна (88° Солнца назад)
    let sun_pos = personality_positions.iter().find(|p| p.planet == HdPlanet::Sun).unwrap();
    let design_jd = astro_calc::find_design_jd(personality_jd, sun_pos.ecliptic_lng);

    // 4. Позиции планет Дизайна
    let design_positions = astro_calc::calc_planet_positions(design_jd);

    // 5. Конвертировать в GatePosition
    let pers_gates: Vec<_> = personality_positions.iter()
        .map(|p| (p.planet, gates::degree_to_gate(p.ecliptic_lng)))
        .collect();
    let des_gates: Vec<_> = design_positions.iter()
        .map(|p| (p.planet, gates::degree_to_gate(p.ecliptic_lng)))
        .collect();

    // 6. Собрать все активированные ворота
    let mut all_active_gates: Vec<u8> = Vec::new();
    for (_, gp) in &pers_gates {
        all_active_gates.push(gp.gate);
    }
    for (_, gp) in &des_gates {
        all_active_gates.push(gp.gate);
    }
    all_active_gates.sort();
    all_active_gates.dedup();

    // 7. Найти каналы
    let active_channels = channels::find_active_channels(&all_active_gates);
    let active_channels = channels::unique_channels(active_channels);

    // 8. Определить центры
    let defined_centers = find_defined_centers(&active_channels);

    // 9. Тип
    let hd_type = determine_type(&defined_centers, &active_channels);

    // 10. Авторитет
    let authority = determine_authority(&defined_centers);

    // 11. Стратегия
    let strategy = determine_strategy(&hd_type);
    let strategy_description = if full { db.strategies.get(&strategy).cloned() } else { None };

    // 12. Профиль
    let pers_sun_gp = pers_gates.iter().find(|(p, _)| *p == HdPlanet::Sun).unwrap();
    let des_sun_gp = des_gates.iter().find(|(p, _)| *p == HdPlanet::Sun).unwrap();
    let profile = format!("{}/{}", pers_sun_gp.1.line, des_sun_gp.1.line);

    // 13. Инкарнационный Крест
    let pers_earth_gp = pers_gates.iter().find(|(p, _)| *p == HdPlanet::Earth).unwrap();
    let des_earth_gp = des_gates.iter().find(|(p, _)| *p == HdPlanet::Earth).unwrap();
    let cross_type = match profile.as_str() {
        "1/3" | "1/4" | "2/4" | "2/5" | "3/5" | "3/6" | "4/6" => "Правоугольный",
        "4/1" => "Джакста-позиции",
        "5/1" | "5/2" | "6/2" | "6/3" => "Левоугольный",
        _ => "Правоугольный", // Fallback
    };
    // Ищем название креста в БД
    let cross_name_opt = find_cross_name_in_db(db, &pers_sun_gp.1.gate.to_string(), cross_type);

    let incarnation_cross = if let Some(ref name) = cross_name_opt {
        format!(
            "{} ({}/{} | {}/{})",
            name,
            pers_sun_gp.1.gate, pers_earth_gp.1.gate,
            des_sun_gp.1.gate, des_earth_gp.1.gate
        )
    } else {
         format!(
            "Крест {} ({}/{} | {}/{})",
            cross_type,
            pers_sun_gp.1.gate, pers_earth_gp.1.gate,
            des_sun_gp.1.gate, des_earth_gp.1.gate
        )
    };

    // Ищем описание креста в БД (только текст)
    let cross_description = if full {
        if let Some(ref name) = cross_name_opt {
            db.crosses.get(name).cloned()
        } else {
            None
        }
    } else {
        None
    };

    // 14. Мотивация (Цвет Солнца Личности)
    let pers_sun_color = pers_sun_gp.1.color;
    let motivation = db.motivation.as_ref()
        .map(|m| {
            let desc = m.colors.get(&pers_sun_color.to_string()).cloned().unwrap_or_default();
            vec![InfoItem {
                label: format!("Цвет {}:", pers_sun_color),
                description: desc,
            }]
        });

    // 15. Среда (Цвет Узлов Дизайна - Северный Узел)
    let des_node_gp = des_gates.iter().find(|(p, _)| *p == HdPlanet::NorthNode);
    let environment = if let Some((_, node)) = des_node_gp {
        db.environment.as_ref().map(|e| {
            let desc = e.colors.get(&node.color.to_string()).cloned().unwrap_or_default();
            vec![InfoItem {
                label: format!("Цвет {}:", node.color),
                description: desc,
            }]
        })
    } else {
        None
    };

    // 16. Диета (Цвет и Тон Солнца Дизайна)
    let des_sun_color = des_sun_gp.1.color;
    let des_sun_tone = des_sun_gp.1.tone;
    let diet = db.diet.as_ref().map(|d| {
        let c_desc = d.colors.get(&des_sun_color.to_string()).cloned().unwrap_or_default();
        let mut items = vec![InfoItem {
            label: format!("Цвет {}:", des_sun_color),
            description: c_desc,
        }];

        if let Some(t) = d.tones.get(&des_sun_tone.to_string()) {
            items.push(InfoItem {
                label: format!("Тон {}:", des_sun_tone),
                description: format!("{} - {}", t.name, t.description),
            });
        } else {
             items.push(InfoItem {
                label: format!("Тон {}:", des_sun_tone),
                description: "".to_string(),
            });
        }
        items
    });

    // 17. Перспектива / Видение (Цвет Узлов Личности)
    let pers_node_gp = pers_gates.iter().find(|(p, _)| *p == HdPlanet::NorthNode);
    let vision = if let Some((_, node)) = pers_node_gp {
        db.vision.as_ref().map(|v| {
            let desc = v.colors.get(&node.color.to_string()).cloned().unwrap_or_default();
            vec![InfoItem {
                label: format!("Цвет {}:", node.color),
                description: desc,
            }]
        })
    } else {
        None
    };

    // 18. Страх (из дизайн-Солнца? обычно ассоциируется с цветом личности, но оставим как было или исправим)
    // В оригинале бралось из des_sun_color. Многие источники связывают страх с Мотивацией (Personality Sun).
    // Поправим на Personality Sun Color (Мотивация - замещение).
    let fear = db.fears.get(&pers_sun_color.to_string()).cloned();

    // 19. Сексуальность (из ворот дизайн-Марса - остаётся как есть)
    let des_mars_gp = des_gates.iter().find(|(p, _)| *p == HdPlanet::Mars);
    let sexuality = des_mars_gp.and_then(|(_, gp)| {
        db.gates.get(&gp.gate.to_string())
            .and_then(|g| g.sexuality.as_ref())
            .map(|s| format!("{}: {}", s.title, s.description))
    });

    // 20. Любовь (из Венеры Личности - остаётся как есть)
    let pers_venus_gp = pers_gates.iter().find(|(p, _)| *p == HdPlanet::Venus);
    let love = pers_venus_gp.and_then(|(_, gp)| {
        let key = format!("{} линия", gp.line);
        db.gates.get(&gp.gate.to_string())
            .and_then(|g| g.lines.get(&key))
            .cloned()
    });

    // Формируем PlanetPosition
    let personality = build_planet_positions(&pers_gates, db, full);
    let design = build_planet_positions(&des_gates, db, full);

    // Формируем каналы
    let channel_infos: Vec<ChannelInfo> = active_channels.iter().map(|ch| {
        let _key1 = ch.key(); // MIN-MAX
        let _key2 = format!("{}-{}", ch.gate_b, ch.gate_a); // MAX-MIN? Or just the other way
        // Determine the other permutation
        let (min, max) = if ch.gate_a < ch.gate_b { (ch.gate_a, ch.gate_b) } else { (ch.gate_b, ch.gate_a) };
        let key_min_max = format!("{}-{}", min, max);
        let key_max_min = format!("{}-{}", max, min);
        
        let ch_data = db.channels.get(&key_min_max).or_else(|| db.channels.get(&key_max_min));
        
        ChannelInfo {
            key: key_min_max.clone(),
            name: ch_data.map(|c| c.name.clone()).unwrap_or_else(|| key_min_max.clone()),
            description: if full { ch_data.map(|c| c.description.clone()) } else { None },
        }
    }).collect();

    // Формируем центры
    let center_infos: Vec<CenterInfo> = Center::all().iter().map(|c| {
        let defined = defined_centers.contains(c);
        let behavior = if full {
            db.centers.get(c.db_key()).map(|cb| {
                if defined {
                    cb.normal_behavior.clone()
                } else {
                    cb.distorted_behavior.clone()
                }
            })
        } else {
            None
        };
        CenterInfo {
            name: c.name_ru().to_string(),
            defined,
            behavior,
        }
    }).collect();

    // Бизнес (от активных ворот)
    let business = if full {
        let mut biz = Vec::new();
        for gate_id in &all_active_gates {
            if let Some(gate_data) = db.gates.get(&gate_id.to_string()) {
                if let Some(b) = &gate_data.business {
                    biz.push(BusinessInfo {
                        gate: *gate_id,
                        title: b.title.clone(),
                        description: Some(b.description.clone()),
                    });
                }
            }
        }
        if biz.is_empty() { None } else { Some(biz) }
    } else {
        None
    };

    HdChart {
        birth_date: format!("{:04}-{:02}-{:02}", year, month, day),
        birth_time: format!("{:02}:{:02}", hour, min),
        utc_offset,
        hd_type: hd_type.clone(),
        type_description: if full { db.types.get(&hd_type).cloned() } else { None },
        profile: profile.clone(),
        profile_description: if full { db.profiles.get(&profile).cloned() } else { None },
        authority: authority.clone(),
        authority_description: if full { db.authorities.get(&authority).cloned() } else { None },
        strategy,
        strategy_description,
        incarnation_cross,
        cross_description,
        personality,
        design,
        channels: channel_infos,
        centers: center_infos,
        business,
        motivation,
        environment,
        diet,
        fear,
        sexuality,
        love,
        vision,
    }
}

fn build_planet_positions(
    positions: &[(HdPlanet, gates::GatePosition)],
    db: &HdDatabase,
    full: bool,
) -> Vec<PlanetPosition> {
    positions.iter().enumerate().map(|(idx, (planet, gp))| {
        let (zodiac_sign, zodiac_degree) = gates::degree_to_zodiac(gp.degree);
        let zodiac_symbol = zodiac_symbol_from_name(&zodiac_sign);
        
        // Gate name is always needed for table now
        let gate_name = db.gates.get(&gp.gate.to_string()).map(|g| g.name.clone());
        
        let (gate_description, line_description) = if full {
            let g_desc = db.gates.get(&gp.gate.to_string()).map(|g| g.description.clone());
            let l_key = format!("{} линия", gp.line);
            let l_desc = db.gates.get(&gp.gate.to_string())
                .and_then(|g| g.lines.get(&l_key))
                .cloned();
            (g_desc, l_desc)
        } else {
            (None, None)
        };

        PlanetPosition {
            planet: planet.name_ru().to_string(),
            index: idx,
            longitude: gp.degree,
            degree: (gp.degree * 100.0).round() / 100.0,
            zodiac_sign,
            zodiac_symbol,
            zodiac_degree: (zodiac_degree * 100.0).round() / 100.0,
            gate: gp.gate,
            line: gp.line,
            color: gp.color,
            tone: gp.tone,
            base: gp.base,
            gate_name,
            gate_description,
            line_description,
        }
    }).collect()
}

fn zodiac_symbol_from_name(name: &str) -> String {
    match name {
        "Овен" => "♈",
        "Телец" => "♉",
        "Близнецы" => "♊",
        "Рак" => "♋",
        "Лев" => "♌",
        "Дева" => "♍",
        "Весы" => "♎",
        "Скорпион" => "♏",
        "Стрелец" => "♐",
        "Козерог" => "♑",
        "Водолей" => "♒",
        "Рыбы" => "♓",
        _ => "",
    }.to_string()
}

fn find_defined_centers(channels: &[ChannelDef]) -> HashSet<Center> {
    let mut defined = HashSet::new();
    for ch in channels {
        defined.insert(ch.center_a);
        defined.insert(ch.center_b);
    }
    defined
}

fn determine_type(defined: &HashSet<Center>, channels: &[ChannelDef]) -> String {
    let has_sacral = defined.contains(&Center::Sacral);
    let _has_throat = defined.contains(&Center::Throat);

    // Проверяем связь мотора с горлом
    let motor_to_throat = has_motor_to_throat_connection(defined, channels);

    if defined.is_empty() {
        "Рефлектор".to_string()
    } else if has_sacral && motor_to_throat {
        "Манифестирующий генератор".to_string()
    } else if has_sacral {
        "Генератор".to_string()
    } else if motor_to_throat {
        "Манифестор".to_string()
    } else {
        "Проектор".to_string()
    }
}

fn has_motor_to_throat_connection(defined: &HashSet<Center>, channels: &[ChannelDef]) -> bool {
    // BFS/DFS от горла до любого мотора через определённые каналы
    if !defined.contains(&Center::Throat) {
        return false;
    }

    let mut visited = HashSet::new();
    let mut stack = vec![Center::Throat];

    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        if current != Center::Throat && current.is_motor() {
            return true;
        }

        // Найти соседей через каналы
        for ch in channels {
            if ch.center_a == current && defined.contains(&ch.center_b) {
                stack.push(ch.center_b);
            }
            if ch.center_b == current && defined.contains(&ch.center_a) {
                stack.push(ch.center_a);
            }
        }
    }

    false
}

fn determine_authority(defined: &HashSet<Center>) -> String {
    if defined.contains(&Center::SolarPlexus) {
        "Эмоциональный (Солнечное сплетение)".to_string()
    } else if defined.contains(&Center::Sacral) {
        "Сакральный".to_string()
    } else if defined.contains(&Center::Spleen) {
        "Селезёночный".to_string()
    } else if defined.contains(&Center::Heart) {
        "Эго / Сердечный".to_string()
    } else if defined.contains(&Center::G) {
        "Самопроецируемый (Джи)".to_string()
    } else if defined.contains(&Center::Throat) {
        "Ментальный".to_string()
    } else {
        "Лунный (без авторитета)".to_string()
    }
}

fn determine_strategy(hd_type: &str) -> String {
    match hd_type {
        "Генератор" | "Манифестирующий генератор" => "Ждать отклика".to_string(),
        "Проектор" => "Ждать приглашения".to_string(),
        "Манифестор" => "Информировать".to_string(),
        "Рефлектор" => "Ждать лунный цикл (29 дней)".to_string(),
        _ => "Неизвестно".to_string(),
    }
}

fn find_cross_name_in_db(
    db: &HdDatabase,
    sun_gate_id: &str,
    cross_type: &str,
) -> Option<String> {
    if let Some(gate_data) = db.gates.get(sun_gate_id) {
        // 1. Exact match contains cross_type
        for cross_name in &gate_data.crosses {
            if cross_name.contains(cross_type) {
                return Some(cross_name.clone());
            }
        }
        // 2. Fallback: returns first if list not empty
        // logic: if we calculated "Right Angle" but DB only has "Juxtaposition" (unlikely) or vice versa,
        // we might want to be careful. But usually Gate 8 has Right and Left.
        if let Some(first) = gate_data.crosses.first() {
             return Some(first.clone());
        }
    }
    None
}
