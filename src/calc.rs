/// HD calculations: type, profile, authority, channels, centers, cross

use crate::astro_calc::{self, HdPlanet};
use crate::data::centers::Center;
use crate::data::channels::{self, ChannelDef};
use crate::data::database::{self, HdDatabase};
use crate::data::gates;
use crate::models::*;
use std::collections::HashSet;

/// Build full chart
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

    // 1. Julian Day for Personality (moment of birth)
    let personality_jd = astro_calc::calc_julian_day(year, month, day, hour, min, utc_offset);

    // 2. Personality planet positions
    let personality_positions = astro_calc::calc_planet_positions(personality_jd);

    // 3. Find Julian Day for Design (88° prior to Sun)
    let sun_pos = personality_positions.iter().find(|p| p.planet == HdPlanet::Sun).unwrap();
    let design_jd = astro_calc::find_design_jd(personality_jd, sun_pos.ecliptic_lng);

    // 4. Design planet positions
    let design_positions = astro_calc::calc_planet_positions(design_jd);

    // 5. Convert to GatePosition
    let pers_gates: Vec<_> = personality_positions.iter()
        .map(|p| (p.planet, gates::degree_to_gate(p.ecliptic_lng)))
        .collect();
    let des_gates: Vec<_> = design_positions.iter()
        .map(|p| (p.planet, gates::degree_to_gate(p.ecliptic_lng)))
        .collect();

    // 6. Collect all active gates
    let mut all_active_gates: Vec<u8> = Vec::new();
    for (_, gp) in &pers_gates {
        all_active_gates.push(gp.gate);
    }
    for (_, gp) in &des_gates {
        all_active_gates.push(gp.gate);
    }
    all_active_gates.sort();
    all_active_gates.dedup();

    // 7. Find channels
    let active_channels = channels::find_active_channels(&all_active_gates);
    let active_channels = channels::unique_channels(active_channels);

    // 8. Determine centers
    let defined_centers = find_defined_centers(&active_channels);

    // 9. Type
    let type_key = determine_type(&defined_centers, &active_channels);
    let type_meta = db.types.get(&type_key);
    let hd_type = type_meta.map(|m| m.name.clone()).unwrap_or_else(|| type_key.clone());
    let type_description = if full { type_meta.map(|m| m.description.clone()) } else { None };

    // 10. Authority
    let authority_key = determine_authority(&defined_centers);
    let authority_meta = db.authorities.get(&authority_key);
    let authority = authority_meta.map(|m| m.name.clone()).unwrap_or_else(|| authority_key.clone());
    let authority_description = if full { authority_meta.map(|m| m.description.clone()) } else { None };

    // 11. Strategy
    // Strategy logic is strictly tied to Type, but texts might differ or not be in DB.
    // We use the Russian texts hardcoded if DB is empty, or try to find them if needed.
    // For now, retaining hardcoded Russian strategy names as per previous implementation,
    // as `strategies` map in DB might be empty.
    let strategy = determine_strategy_localized(&type_key);
    // Description: if `strategies` map has keys matching `type_key` (e.g. "generator"), use it.
    let strategy_description = if full { db.strategies.get(&type_key).cloned() } else { None };

    // 12. Profile
    let pers_sun_gp = pers_gates.iter().find(|(p, _)| *p == HdPlanet::Sun).unwrap();
    let des_sun_gp = des_gates.iter().find(|(p, _)| *p == HdPlanet::Sun).unwrap();
    let profile_key = format!("{}/{}", pers_sun_gp.1.line, des_sun_gp.1.line);
    let profile_meta = db.profiles.get(&profile_key);
    let profile = profile_meta.map(|m| m.name.clone()).unwrap_or_else(|| profile_key.clone());
    let profile_description = if full { profile_meta.map(|m| m.description.clone()) } else { None };

    // 13. Incarnation Cross
    let pers_earth_gp = pers_gates.iter().find(|(p, _)| *p == HdPlanet::Earth).unwrap();
    let des_earth_gp = des_gates.iter().find(|(p, _)| *p == HdPlanet::Earth).unwrap();
    
    // Determine Angle (English keys)
    let angle_key = match profile_key.as_str() {
        "1/3" | "1/4" | "2/4" | "2/5" | "3/5" | "3/6" | "4/6" => "right_angle",
        "4/1" => "juxtaposition",
        "5/1" | "5/2" | "6/2" | "6/3" => "left_angle",
        _ => "right_angle", // Fallback
    };
    
    // Search cross key in DB
    let cross_db_key_opt = find_cross_key_in_db(db, &pers_sun_gp.1.gate.to_string(), angle_key);
    
    // Get localized name and description
    let (cross_name, cross_desc) = if let Some(ref key) = cross_db_key_opt {
        let meta = db.crosses.get(key);
        (
            meta.map(|m| m.name.clone()), 
            if full { meta.map(|m| m.description.clone()) } else { None }
        )
    } else {
        (None, None)
    };

    // Use description from DB if found
    let cross_description = cross_desc;

    let incarnation_cross = if let Some(name) = cross_name {
        format!(
            "{} ({}/{} | {}/{})",
            name,
            pers_sun_gp.1.gate, pers_earth_gp.1.gate,
            des_sun_gp.1.gate, des_earth_gp.1.gate
        )
    } else {
        // Fallback name generation (Localized)
        let angle_name = match angle_key {
            "right_angle" => rust_i18n::t!("angle.right_angle").to_string(),
            "juxtaposition" => rust_i18n::t!("angle.juxtaposition").to_string(),
            "left_angle" => rust_i18n::t!("angle.left_angle").to_string(),
            _ => "".to_string(),
        };
        
        rust_i18n::t!("cross.default_fmt", 
            angle = angle_name,
            p_sun = pers_sun_gp.1.gate,
            p_earth = pers_earth_gp.1.gate,
            d_sun = des_sun_gp.1.gate,
            d_earth = des_earth_gp.1.gate
        ).to_string()
    };

    // 14. Motivation (Personality Sun Color)
    let pers_sun_color = pers_sun_gp.1.color;
    let motivation = db.motivation.as_ref()
        .map(|m| {
            let desc = m.colors.get(&pers_sun_color.to_string()).cloned().unwrap_or_default();
            vec![InfoItem {
                label: format!("{} {}:", rust_i18n::t!("cli.label.color"), pers_sun_color),
                description: desc,
            }]
        });

    // 15. Environment (Design Nodes Color - North Node)
    let des_node_gp = des_gates.iter().find(|(p, _)| *p == HdPlanet::NorthNode);
    let environment = if let Some((_, node)) = des_node_gp {
        db.environment.as_ref().map(|e| {
            let desc = e.colors.get(&node.color.to_string()).cloned().unwrap_or_default();
            vec![InfoItem {
                label: format!("{} {}:", rust_i18n::t!("cli.label.color"), node.color),
                description: desc,
            }]
        })
    } else {
        None
    };

    // 16. Diet (Design Sun Color and Tone)
    let des_sun_color = des_sun_gp.1.color;
    let des_sun_tone = des_sun_gp.1.tone;
    let diet = db.diet.as_ref().map(|d| {
        let c_desc = d.colors.get(&des_sun_color.to_string()).cloned().unwrap_or_default();
        let mut items = vec![InfoItem {
            label: format!("{} {}:", rust_i18n::t!("cli.label.color"), des_sun_color),
            description: c_desc,
        }];

        if let Some(t_desc) = d.tones.get(&des_sun_tone.to_string()) {
            items.push(InfoItem {
                label: format!("{} {}:", rust_i18n::t!("cli.label.tone"), des_sun_tone),
                description: t_desc.clone(),
            });
        } else {
             items.push(InfoItem {
                label: format!("{} {}:", rust_i18n::t!("cli.label.tone"), des_sun_tone),
                description: "".to_string(),
            });
        }
        items
    });

    // 17. Perspective / Vision (Personality Nodes Color)
    let pers_node_gp = pers_gates.iter().find(|(p, _)| *p == HdPlanet::NorthNode);
    let vision = if let Some((_, node)) = pers_node_gp {
        db.vision.as_ref().map(|v| {
            let desc = v.colors.get(&node.color.to_string()).cloned().unwrap_or_default();
            vec![InfoItem {
                label: format!("{} {}:", rust_i18n::t!("cli.label.color"), node.color),
                description: desc,
            }]
        })
    } else {
        None
    };

    // 18. Fear, Sexuality, Love (from all active gates)
    let mut fears = Vec::new();
    let mut sexualities = Vec::new();
    let mut loves = Vec::new();

    // Add Motivation Fear (if present)
    if let Some(f) = db.fears.get(&pers_sun_color.to_string()) {
        fears.push(InfoItem {
            label: format!("{} {}:", rust_i18n::t!("cli.label.motivation"), pers_sun_color),
            description: f.clone(),
        });
    }

    // Sort active gates to ensure consistent order
    // (all_active_gates is already sorted)
    for gate_id in &all_active_gates {
        if let Some(gate_data) = db.gates.get(&gate_id.to_string()) {
            // Label: "Gate X (Name):"
            let gate_name = &gate_data.name;
            let gate_label = format!("{} {} ({}):", rust_i18n::t!("cli.label.gate"), gate_id, gate_name);
            
            if let Some(f) = &gate_data.fear {
                fears.push(InfoItem {
                    label: gate_label.clone(),
                    description: f.clone(),
                });
            }
            if let Some(s) = &gate_data.sexuality {
                sexualities.push(InfoItem {
                    label: gate_label.clone(),
                    description: s.clone(),
                });
            }
            if let Some(l) = &gate_data.love {
                loves.push(InfoItem {
                    label: gate_label.clone(),
                    description: l.clone(),
                });
            }
        }
    }

    let fear = if fears.is_empty() { None } else { Some(fears) };
    let sexuality = if sexualities.is_empty() { None } else { Some(sexualities) };
    let love = if loves.is_empty() { None } else { Some(loves) };

    // Form PlanetPosition
    let personality = build_planet_positions(&pers_gates, db, full);
    let design = build_planet_positions(&des_gates, db, full);

    // Form channels
    let channel_infos: Vec<ChannelInfo> = active_channels.iter().map(|ch| {
        // Gates are always sorted min-max in ChannelDef if from `channels::all_channels()`?
        // But here `ch` is from `channels.rs` defs.
        // Let's ensure consistent key lookup. 
        // In `channels.rs`, `gates` map is "GateA-GateB" where A < B usually?
        // Let's assume standard sorting Min-Max.
        let (min, max) = if ch.gate_a < ch.gate_b { (ch.gate_a, ch.gate_b) } else { (ch.gate_b, ch.gate_a) };
        let key_min_max = format!("{}-{}", min, max);
        let key_max_min = format!("{}-{}", max, min);
        
        let ch_data = db.channels.get(&key_min_max).or_else(|| db.channels.get(&key_max_min));
        
        ChannelInfo {
            key: key_min_max.clone(),
            name: ch_data.and_then(|c| c.name.clone()).unwrap_or_else(|| key_min_max.clone()),
            description: if full { ch_data.map(|c| c.description.clone()) } else { None },
        }
    }).collect();

    // Form centers
    let center_infos: Vec<CenterInfo> = Center::all().iter().map(|c| {
        let defined = defined_centers.contains(c);
        let center_key = c.key(); // English key: "head", "ajna"
        let center_data_opt = db.centers.get(center_key);
        
        let name = center_data_opt.map(|d| d.name.clone()).unwrap_or_else(|| center_key.to_string());
        
        let (behavior_normal, behavior_distorted) = if full {
            if let Some(cb) = center_data_opt {
                (Some(cb.normal.clone()), Some(cb.distorted.clone()))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };
        
        CenterInfo {
            name,
            defined,
            behavior_normal,
            behavior_distorted,
        }
    }).collect();

    // Business (from active gates)
    let business = if full {
        let mut biz = Vec::new();
        for gate_id in &all_active_gates {
            if let Some(gate_data) = db.gates.get(&gate_id.to_string()) {
                if let Some(b) = &gate_data.business {
                    biz.push(BusinessInfo {
                        gate: *gate_id,
                        title: b.clone(), // Now it's a String
                        description: None, // JSON "business" is just a String, no description field
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
        hd_type,
        type_description,
        profile,
        profile_description,
        authority,
        authority_description,
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
        let (zodiac_key, zodiac_degree) = gates::degree_to_zodiac(gp.degree);
        let zodiac_symbol = zodiac_symbol_from_key(&zodiac_key);
        // Localize
        let zodiac_key_str = format!("zodiac.{}", zodiac_key);
        let zodiac_sign = rust_i18n::t!(&zodiac_key_str).to_string();
        
        let gate_name = db.gates.get(&gp.gate.to_string()).map(|g| g.name.clone());
        
        let (gate_description, line_description) = if full {
            let g_desc = db.gates.get(&gp.gate.to_string()).map(|g| g.description.clone());
            let l_key = format!("{}", gp.line); // JSON lines are "1", "2"...
            let l_desc = db.gates.get(&gp.gate.to_string())
                .and_then(|g| g.lines.get(&l_key))
                .cloned();
            (g_desc, l_desc)
        } else {
            (None, None)
        };

        PlanetPosition {
            planet: planet.name(),
            index: idx,
            longitude: gp.degree,
            degree: (gp.degree * 100.0).round() / 100.0,
            zodiac_sign,
            zodiac_symbol,
            planet_symbol: planet.symbol(),
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

fn zodiac_symbol_from_key(key: &str) -> String {
    // Using \u{FE0E} to force text presentation (no emoji color/frame)
    match key {
        "aries" => "♈\u{FE0E}",
        "taurus" => "♉\u{FE0E}",
        "gemini" => "♊\u{FE0E}",
        "cancer" => "♋\u{FE0E}",
        "leo" => "♌\u{FE0E}",
        "virgo" => "♍\u{FE0E}",
        "libra" => "♎\u{FE0E}",
        "scorpio" => "♏\u{FE0E}",
        "sagittarius" => "♐\u{FE0E}",
        "capricorn" => "♑\u{FE0E}",
        "aquarius" => "♒\u{FE0E}",
        "pisces" => "♓\u{FE0E}",
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

// Returns snake_case key (e.g. "generator")
fn determine_type(defined: &HashSet<Center>, channels: &[ChannelDef]) -> String {
    let has_sacral = defined.contains(&Center::Sacral);
    let _has_throat = defined.contains(&Center::Throat);

    // Check motor to throat connection
    let motor_to_throat = has_motor_to_throat_connection(defined, channels);

    if defined.is_empty() {
        "reflector".to_string()
    } else if has_sacral && motor_to_throat {
        "manifesting_generator".to_string()
    } else if has_sacral {
        "generator".to_string()
    } else if motor_to_throat {
        "manifestor".to_string()
    } else {
        "projector".to_string()
    }
}

fn has_motor_to_throat_connection(defined: &HashSet<Center>, channels: &[ChannelDef]) -> bool {
    // BFS/DFS from throat to any motor through defined channels
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

        // Find neighbors through channels
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

// Returns snake_case key
fn determine_authority(defined: &HashSet<Center>) -> String {
    if defined.contains(&Center::SolarPlexus) {
        "emotional".to_string()
    } else if defined.contains(&Center::Sacral) {
        "sacral".to_string()
    } else if defined.contains(&Center::Spleen) {
        "splenic".to_string()
    } else if defined.contains(&Center::Heart) {
        "ego".to_string() 
    } else if defined.contains(&Center::G) {
        "self_projected".to_string()
    } else if defined.contains(&Center::Throat) {
        "mental".to_string()
    } else {
        "lunar".to_string()
    }
}

fn determine_strategy_localized(hd_type_key: &str) -> String {
    // actually we have specific keys in YAML
    match hd_type_key {
        "generator" => rust_i18n::t!("strategy.generator").to_string(),
        "manifesting_generator" => rust_i18n::t!("strategy.manifesting_generator").to_string(),
        "projector" => rust_i18n::t!("strategy.projector").to_string(),
        "manifestor" => rust_i18n::t!("strategy.manifestor").to_string(),
        "reflector" => rust_i18n::t!("strategy.reflector").to_string(),
        _ => rust_i18n::t!("strategy.unknown").to_string(),
    }
}

fn find_cross_key_in_db(
    db: &HdDatabase,
    sun_gate_id: &str,
    angle_key_part: &str, // "right_angle", "left_angle", "juxtaposition"
) -> Option<String> {
    if let Some(gate_data) = db.gates.get(sun_gate_id) {
        // gate_data.crosses is a list of KEYS like "right_angle_cross_of_the_sphinx"
        for cross_key in &gate_data.crosses {
            if cross_key.contains(angle_key_part) {
                return Some(cross_key.clone());
            }
        }
        // Fallback: returns first if list not empty
        if let Some(first) = gate_data.crosses.first() {
             return Some(first.clone());
        }
    }
    None
}
