/// Circuit weight scoring for Human Design charts.
///
/// Calculates how strongly each circuit/subcircuit is represented in a chart
/// based on:
/// 1. Planetary weights (Sun/Earth = highest, Uranus/Pluto = lowest)
/// 2. Bonus for multiple planets in the same sub-circuit
/// 3. Bonus for closed (active) channels in a sub-circuit
use crate::astro_calc::HdPlanet;
use crate::data::channels::ChannelDef;
use crate::data::database::HdDatabase;
use crate::data::gates;
use crate::models::CircuitScoreItem;
use std::collections::HashMap;

// ─────────────────────────────────────────────
//  Planet weights (HD canonical order)
// ─────────────────────────────────────────────

fn planet_weight(planet: HdPlanet) -> f64 {
    match planet {
        HdPlanet::Sun => 10.0,
        HdPlanet::Earth => 10.0, // Same weight as Sun
        HdPlanet::Moon => 9.0,
        HdPlanet::NorthNode => 8.0,
        HdPlanet::SouthNode => 8.0,
        HdPlanet::Jupiter => 9.0,
        HdPlanet::Venus => 8.0,
        HdPlanet::Mars => 8.0,
        HdPlanet::Mercury => 7.0,
        HdPlanet::Saturn => 7.0,
        HdPlanet::Uranus => 6.0,
        HdPlanet::Neptune => 6.0,
        HdPlanet::Pluto => 6.0,
    }
}

// Bonus per each additional planet (beyond first) in the same sub-circuit
const MULTI_PLANET_BONUS: f64 = 1.2;
// Bonus per closed (active) channel in a sub-circuit
const CLOSED_CHANNEL_BONUS: f64 = 1.4;

// ─────────────────────────────────────────────
//  Sub-circuit lookup from gate data
// ─────────────────────────────────────────────

/// Returns (circuit, sub_circuit) for a gate_id, or ("unknown", "unknown")
fn gate_circuit_info(gate_id: u8, db: &HdDatabase) -> (String, String) {
    if let Some(gate) = db.gates.get(&gate_id.to_string()) {
        let circuit = gate
            .circuit
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let sub = gate
            .sub_circuit
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        (circuit, sub)
    } else {
        ("unknown".to_string(), "unknown".to_string())
    }
}

fn channel_sub_circuit(ch: &ChannelDef, db: &HdDatabase) -> (String, String) {
    gate_circuit_info(ch.gate_a, db)
}

pub fn calculate_circuit_scores(
    pers_gates: &[(HdPlanet, gates::GatePosition)],
    des_gates: &[(HdPlanet, gates::GatePosition)],
    active_channels: &[ChannelDef],
    db: &HdDatabase,
) -> Vec<CircuitScoreItem> {
    let mut scores: HashMap<(String, String), (f64, usize, usize)> = HashMap::new();

    let mut add_planet = |gate_id: u8, planet: HdPlanet| {
        let (circuit, sub) = gate_circuit_info(gate_id, db);
        if circuit == "unknown" {
            return;
        }
        let key = (circuit, sub);
        let entry = scores.entry(key).or_insert((0.0, 0, 0));
        entry.0 += planet_weight(planet);
        entry.1 += 1;
    };

    for (planet, gp) in pers_gates {
        add_planet(gp.gate, *planet);
    }
    for (planet, gp) in des_gates {
        add_planet(gp.gate, *planet);
    }

    for ch in active_channels {
        let (circuit, sub) = channel_sub_circuit(ch, db);
        if circuit == "unknown" {
            continue;
        }
        let entry = scores.entry((circuit, sub)).or_insert((0.0, 0, 0));
        entry.2 += 1;
    }

    let mut items: Vec<CircuitScoreItem> = scores
        .into_iter()
        .map(
            |((circuit, sub_circuit), (base, planet_count, channel_count))| {
                let multi_bonus = if planet_count > 1 {
                    MULTI_PLANET_BONUS.powi((planet_count - 1) as i32)
                } else {
                    1.0
                };
                let channel_bonus = if channel_count > 0 {
                    CLOSED_CHANNEL_BONUS.powi(channel_count as i32)
                } else {
                    1.0
                };
                let mut total = base * multi_bonus * channel_bonus;

                let total_sub_channels = db
                    .channels
                    .values()
                    .filter(|ch| {
                        ch.circuit.as_ref() == Some(&circuit)
                            && ch.sub_circuit.as_ref() == Some(&sub_circuit)
                    })
                    .count();

                if total_sub_channels > 0 {
                    total /= total_sub_channels as f64;
                }

                let (circuit_name, circuit_desc, sub_circuit_name, description) =
                    if let Some(circ) = db.circuits.get(&circuit) {
                        let c_name = circ.name.clone();
                        let c_desc = circ.description.clone();
                        if let Some(sub_circ) = circ.sub_circuits.get(&sub_circuit) {
                            (
                                c_name,
                                c_desc,
                                sub_circ.name.clone(),
                                sub_circ.description.clone(),
                            )
                        } else {
                            (c_name, c_desc, sub_circuit.clone(), "".to_string())
                        }
                    } else {
                        (
                            circuit.clone(),
                            "".to_string(),
                            sub_circuit.clone(),
                            "".to_string(),
                        )
                    };

                CircuitScoreItem {
                    circuit,
                    circuit_name,
                    circuit_description: circuit_desc,
                    sub_circuit,
                    sub_circuit_name,
                    score: (total * 100.0).round() / 100.0,
                    planet_count,
                    channel_count,
                    description,
                }
            },
        )
        .collect();

    items.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.circuit.cmp(&b.circuit))
            .then_with(|| a.sub_circuit.cmp(&b.sub_circuit))
    });

    items
}

pub fn group_by_circuit<'a>(
    items: &'a [CircuitScoreItem],
) -> Vec<(String, String, String, f64, Vec<&'a CircuitScoreItem>)> {
    let mut circuit_totals: HashMap<String, (String, String, f64)> = HashMap::new();
    for item in items {
        let entry = circuit_totals
            .entry(item.circuit.clone())
            .or_insert_with(|| {
                (
                    item.circuit_name.clone(),
                    item.circuit_description.clone(),
                    0.0,
                )
            });
        entry.2 += item.score;
    }

    let mut circuits: Vec<(String, String, String, f64)> = circuit_totals
        .into_iter()
        .map(|(k, (name, desc, total))| (k, name, desc, total))
        .collect();
    circuits.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));

    circuits
        .into_iter()
        .map(|(circuit_key, circuit_name, circuit_desc, total)| {
            let subs: Vec<&CircuitScoreItem> =
                items.iter().filter(|i| i.circuit == circuit_key).collect();
            (
                circuit_key,
                circuit_name,
                circuit_desc,
                (total * 100.0).round() / 100.0,
                subs,
            )
        })
        .collect()
}
