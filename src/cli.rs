use crate::models::HdChart;
/// CLI interface: arguments, output formatting
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use comfy_table::{
    presets, Attribute, Cell, Color as TableColor, ColumnConstraint, ContentArrangement, Table,
};
use terminal_size::{terminal_size, Width};
use textwrap::Options;

/// Output format
#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    /// Formatted table in terminal
    Table,
    /// JSON format
    Json,
    /// YAML format
    Yaml,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Manage configuration
    Config {
        /// Set default language (en, ru, es)
        #[arg(long)]
        set_lang: Option<String>,
    },
}

/// Human Design CLI — Human Design chart calculation
///
/// Calculates full Human Design chart by date, time of birth
/// and time zone (UTC offset). Supports output in table,
/// JSON or YAML format. Use --short to hide detailed descriptions.
#[derive(Parser, Debug)]
#[command(name = "hd-cli")]
#[command(version = "0.1.0")]
#[command(about = "Human Design CLI — расчёт карты Дизайна Человека")]
#[command(
    long_about = "Рассчитывает полную карту HD по дате/времени рождения и UTC-смещению.\n\nПример:\n  hd-cli --date 1990-05-15 --time 14:30 --utc +3\n  hd-cli --date 1990-05-15 --time 14:30 --utc +3 --format json\n  hd-cli --date 1990-05-15 --time 14:30 --utc +3 --short"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Date of birth in YYYY-MM-DD format (e.g. 1990-05-15)
    #[arg(short = 'd', long)]
    pub date: Option<String>,

    /// Time of birth in HH:MM format (e.g. 14:30)
    #[arg(short = 't', long)]
    pub time: Option<String>,

    /// Time zone as UTC offset (e.g. +3, -5, +5.5)
    #[arg(short = 'u', long)]
    pub utc: Option<String>,

    /// Output format: table (default), json, yaml
    #[arg(short = 'f', long, default_value = "table")]
    pub format: OutputFormat,

    /// Short output (hide detailed descriptions of gates, lines, channels and centers)
    #[arg(long)]
    pub short: bool,

    /// Description language (default: ru). Determines data file gates_database_{lang}.json
    #[arg(short = 'l', long)]
    pub lang: Option<String>,

    /// Save output to file. If filename is not specified, it will be generated automatically.
    #[arg(long, num_args(0..=1), default_missing_value = "default")]
    pub save: Option<String>,
}

/// Parse date from YYYY-MM-DD string
pub fn parse_date(s: &str) -> Result<(i32, u8, u8), String> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return Err(rust_i18n::t!(
            "error.parse_date",
            error = format!("'{}'. Expected YYYY-MM-DD", s)
        )
        .to_string());
    }
    let year: i32 = parts[0].parse().map_err(|_| {
        rust_i18n::t!(
            "error.parse_date",
            error = format!("Invalid year: '{}'", parts[0])
        )
        .to_string()
    })?;
    let month: u8 = parts[1].parse().map_err(|_| {
        rust_i18n::t!(
            "error.parse_date",
            error = format!("Invalid month: '{}'", parts[1])
        )
        .to_string()
    })?;
    let day: u8 = parts[2].parse().map_err(|_| {
        rust_i18n::t!(
            "error.parse_date",
            error = format!("Invalid day: '{}'", parts[2])
        )
        .to_string()
    })?;

    if month < 1 || month > 12 {
        return Err(rust_i18n::t!(
            "error.parse_date",
            error = format!("Month must be 1-12, got: {}", month)
        )
        .to_string());
    }
    if day < 1 || day > 31 {
        return Err(rust_i18n::t!(
            "error.parse_date",
            error = format!("Day must be 1-31, got: {}", day)
        )
        .to_string());
    }
    Ok((year, month, day))
}

/// Parse time from HH:MM string
pub fn parse_time(s: &str) -> Result<(u8, u8), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(rust_i18n::t!(
            "error.parse_time",
            error = format!("'{}'. Expected HH:MM", s)
        )
        .to_string());
    }
    let hour: u8 = parts[0].parse().map_err(|_| {
        rust_i18n::t!(
            "error.parse_time",
            error = format!("Invalid hour: '{}'", parts[0])
        )
        .to_string()
    })?;
    let min: u8 = parts[1].parse().map_err(|_| {
        rust_i18n::t!(
            "error.parse_time",
            error = format!("Invalid minute: '{}'", parts[1])
        )
        .to_string()
    })?;

    if hour > 23 {
        return Err(rust_i18n::t!(
            "error.parse_time",
            error = format!("Hour must be 0-23, got: {}", hour)
        )
        .to_string());
    }
    if min > 59 {
        return Err(rust_i18n::t!(
            "error.parse_time",
            error = format!("Minute must be 0-59, got: {}", min)
        )
        .to_string());
    }
    Ok((hour, min))
}

/// Parse UTC offset from string (+3, -5, +5.5)
pub fn parse_utc_offset(s: &str) -> Result<f64, String> {
    let s = s.trim();
    let offset: f64 = s.parse().map_err(|_| {
        rust_i18n::t!(
            "error.parse_utc",
            error = format!("'{}'. Expected number, e.g. +3, -5", s)
        )
        .to_string()
    })?;
    if offset < -12.0 || offset > 14.0 {
        return Err(rust_i18n::t!(
            "error.parse_utc",
            error = format!("Offset must be -12 to +14, got: {}", offset)
        )
        .to_string());
    }
    Ok(offset)
}

/// Generate chart output string
pub fn generate_output(chart: &HdChart, format: &OutputFormat, plain: bool) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(chart).unwrap(),
        OutputFormat::Yaml => serde_yaml::to_string(chart).unwrap(),
        OutputFormat::Table => build_table_string(chart, plain),
    }
}

// Deprecated in favor of generate_output + println! in main
pub fn output_chart(chart: &HdChart, format: &OutputFormat) {
    println!("{}", generate_output(chart, format, false));
}

use std::fmt::Write;

fn build_table_string(chart: &HdChart, plain: bool) -> String {
    let mut out = String::new();

    // Disable colors globally for colored if plain=true
    if plain {
        colored::control::set_override(false);
    }

    // Header
    writeln!(
        out,
        "\n{}",
        "═══════════════════════════════════════════════════════════════".truecolor(95, 158, 160)
    )
    .unwrap();
    writeln!(
        out,
        "      {}",
        rust_i18n::t!("cli.header").truecolor(255, 255, 255).bold()
    )
    .unwrap();
    writeln!(
        out,
        "{}",
        "═══════════════════════════════════════════════════════════════".truecolor(95, 158, 160)
    )
    .unwrap();

    // Main information
    // Main information
    writeln!(
        out,
        "\n{}",
        rust_i18n::t!("cli.section.main_info")
            .truecolor(95, 158, 160)
            .bold()
    )
    .unwrap();
    writeln!(out).unwrap(); // Spacing

    let label_color = |s: &str| s.truecolor(255, 160, 122); // Soft Coral
    let value_color = |s: &str| s.truecolor(255, 215, 0); // Gold
    let desc_color = colored::Color::TrueColor {
        r: 230,
        g: 228,
        b: 208,
    }; // Beige

    // Helper for conditional table cell formatting
    let add_style = |cell: Cell, color: TableColor, bold: bool| -> Cell {
        if plain {
            cell
        } else {
            let mut c = cell.fg(color);
            if bold {
                c = c.add_attribute(Attribute::Bold);
            }
            c
        }
    };

    writeln!(
        out,
        "  {} {} {} UTC{}",
        label_color(&rust_i18n::t!("cli.label.date")),
        value_color(&chart.birth_date),
        value_color(&chart.birth_time),
        value_color(&format!("{:+}", chart.utc_offset))
    )
    .unwrap();
    writeln!(out).unwrap(); // Empty line after Date for spacing

    writeln!(
        out,
        "  {} {}",
        label_color(&rust_i18n::t!("cli.label.type")),
        value_color(&chart.hd_type).bold()
    )
    .unwrap();
    if let Some(ref desc) = chart.type_description {
        write_wrapped(&mut out, desc, 4, Some(desc_color), false);
    }
    writeln!(out).unwrap(); // Empty line after item

    writeln!(
        out,
        "  {} {}",
        label_color(&rust_i18n::t!("cli.label.profile")),
        value_color(&chart.profile).bold()
    )
    .unwrap();
    if let Some(ref desc) = chart.profile_description {
        write_wrapped(&mut out, desc, 4, Some(desc_color), false);
    }
    writeln!(out).unwrap(); // Empty line after item

    writeln!(
        out,
        "  {} {}",
        label_color(&rust_i18n::t!("cli.label.authority")),
        value_color(&chart.authority).bold()
    )
    .unwrap();
    if let Some(ref desc) = chart.authority_description {
        write_wrapped(&mut out, desc, 4, Some(desc_color), false);
    }
    writeln!(out).unwrap(); // Empty line after item

    writeln!(
        out,
        "  {} {}",
        label_color(&rust_i18n::t!("cli.label.strategy")),
        value_color(&chart.strategy).bold()
    )
    .unwrap();
    if let Some(ref desc) = chart.strategy_description {
        write_wrapped(&mut out, desc, 4, Some(desc_color), false);
    }
    writeln!(out).unwrap(); // Empty line after item

    writeln!(
        out,
        "  {} {}",
        label_color(&rust_i18n::t!("cli.label.cross")),
        value_color(&chart.incarnation_cross).bold()
    )
    .unwrap();
    if let Some(ref desc) = chart.cross_description {
        write_wrapped(&mut out, desc, 4, Some(desc_color), false);
    }
    writeln!(out).unwrap(); // Empty line after item

    // Business
    if let Some(ref biz) = chart.business {
        write_gate_section_items(&mut out, &rust_i18n::t!("cli.section.business"), biz);
    }

    // 4. CHANNELS (Moved here, after Business)
    if !chart.channels.is_empty() {
        writeln!(
            out,
            "\n{}",
            rust_i18n::t!("cli.section.channels")
                .truecolor(95, 158, 160)
                .bold()
        )
        .unwrap();
        writeln!(out).unwrap(); // Отступ

        let has_descriptions = chart.channels.iter().any(|ch| ch.description.is_some());

        let mut table = Table::new();
        table
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);

        let mut headers = vec![
            add_style(
                Cell::new(&rust_i18n::t!("cli.label.channel")),
                TableColor::Rgb {
                    r: 255,
                    g: 160,
                    b: 122,
                },
                true,
            ), // Coral
            add_style(
                Cell::new(&rust_i18n::t!("cli.label.name")),
                TableColor::Rgb {
                    r: 255,
                    g: 215,
                    b: 0,
                },
                true,
            ), // Gold
        ];
        if has_descriptions {
            headers.push(add_style(
                Cell::new(&rust_i18n::t!("cli.label.description")),
                TableColor::Rgb {
                    r: 255,
                    g: 160,
                    b: 122,
                },
                true,
            )); // Coral
        }
        table.set_header(headers);

        for ch in &chart.channels {
            let mut row = vec![
                add_style(
                    Cell::new(&ch.key),
                    TableColor::Rgb {
                        r: 95,
                        g: 158,
                        b: 160,
                    },
                    false,
                ),
                add_style(
                    Cell::new(&ch.name),
                    TableColor::Rgb {
                        r: 255,
                        g: 215,
                        b: 0,
                    },
                    true,
                ), // Gold
            ];
            if has_descriptions {
                let desc = ch.description.clone().unwrap_or_default();
                row.push(add_style(
                    Cell::new(&desc),
                    TableColor::Rgb {
                        r: 230,
                        g: 228,
                        b: 208,
                    },
                    false,
                ));
            }
            table.add_row(row);
        }
        writeln!(out, "{}", table).unwrap();
    }

    // 5. Planets (General table) (Now here)
    write_combined_planet_table(&mut out, &chart.design, &chart.personality, plain);

    // Centers
    writeln!(
        out,
        "\n{}",
        rust_i18n::t!("cli.section.centers")
            .truecolor(95, 158, 160)
            .bold()
    )
    .unwrap();
    writeln!(out).unwrap(); // Spacing

    let mut table = Table::new();
    table
        .load_preset(presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        add_style(
            Cell::new(&rust_i18n::t!("cli.label.center")),
            TableColor::Rgb {
                r: 255,
                g: 160,
                b: 122,
            },
            true,
        ),
        add_style(
            Cell::new(&rust_i18n::t!("cli.label.status")),
            TableColor::Rgb {
                r: 255,
                g: 160,
                b: 122,
            },
            true,
        ),
    ]);
    for center in &chart.centers {
        let status = if center.defined {
            format!("● {}", rust_i18n::t!("cli.label.defined"))
        } else {
            format!("○ {}", rust_i18n::t!("cli.label.open"))
        };
        let height_color = if center.defined {
            TableColor::Rgb {
                r: 255,
                g: 215,
                b: 0,
            }
        } else {
            TableColor::DarkGrey
        }; // Gold for defined
        let color = height_color;

        // Combine behavior descriptions if available
        let content = if let (Some(ref norm), Some(ref dist)) =
            (&center.behavior_normal, &center.behavior_distorted)
        {
            format!("{}\n\n{}", norm, dist)
        } else if let Some(ref beh) = center
            .behavior_normal
            .as_ref()
            .or(center.behavior_distorted.as_ref())
        {
            // Fallback if only one exists (unlikely given calc.rs logic)
            beh.to_string()
        } else {
            status.to_string()
        };

        table.add_row(vec![
            add_style(Cell::new(&center.name), color, true),
            add_style(
                Cell::new(&content),
                TableColor::Rgb {
                    r: 230,
                    g: 228,
                    b: 208,
                },
                false,
            ),
        ]);
    }
    writeln!(out, "{}", table).unwrap();

    // Additional information
    let has_extra = chart.motivation.is_some()
        || chart.environment.is_some()
        || chart.diet.is_some()
        || chart.vision.is_some();

    let is_full_mode = chart.type_description.is_some();

    // Fear Section
    if let Some(ref items) = chart.fear {
        write_gate_section_items(&mut out, &rust_i18n::t!("cli.section.fear"), items);
    }

    // Sexuality Section
    if let Some(ref items) = chart.sexuality {
        write_gate_section_items(&mut out, &rust_i18n::t!("cli.section.sexuality"), items);
    }

    // Love Section
    if let Some(ref items) = chart.love {
        write_gate_section_items(&mut out, &rust_i18n::t!("cli.section.love"), items);
    }

    if has_extra && is_full_mode {
        writeln!(
            out,
            "\n{}",
            rust_i18n::t!("cli.section.extra")
                .truecolor(95, 158, 160)
                .bold()
        )
        .unwrap();
        writeln!(out).unwrap(); // Spacing

        if let Some(ref m) = chart.motivation {
            write_info_items(&mut out, &rust_i18n::t!("cli.label.motivation"), m);
        }
        if let Some(ref v) = chart.vision {
            write_info_items(&mut out, &rust_i18n::t!("cli.label.vision"), v);
        }
        if let Some(ref e) = chart.environment {
            write_info_items(&mut out, &rust_i18n::t!("cli.label.environment"), e);
        }
        if let Some(ref d) = chart.diet {
            write_info_items(&mut out, &rust_i18n::t!("cli.label.diet"), d);
        }
    }

    out
}

fn write_info_items(out: &mut String, title: &str, items: &[crate::models::InfoItem]) {
    writeln!(out, "  {}", title.truecolor(255, 215, 0)).unwrap(); // Gold Title

    let label_color = colored::Color::TrueColor {
        r: 255,
        g: 160,
        b: 122,
    };
    let desc_color = colored::Color::TrueColor {
        r: 230,
        g: 228,
        b: 208,
    };

    for item in items {
        writeln!(out, "    {}", item.label.color(label_color)).unwrap();
        if !item.description.is_empty() {
            write_wrapped(out, &item.description, 6, Some(desc_color), false);
        }
    }
}

fn write_combined_planet_table(
    out: &mut String,
    design: &[crate::models::PlanetPosition],
    personality: &[crate::models::PlanetPosition],
    plain: bool,
) {
    writeln!(
        out,
        "\n{}",
        rust_i18n::t!("cli.section.planets")
            .truecolor(95, 158, 160)
            .bold()
    )
    .unwrap();

    let tc_label = TableColor::Rgb {
        r: 255,
        g: 160,
        b: 122,
    };
    let tc_white = TableColor::White;

    let add_style = |cell: Cell, color: TableColor, bold: bool| -> Cell {
        if plain {
            cell
        } else {
            let mut c = cell.fg(color);
            if bold {
                c = c.add_attribute(Attribute::Bold);
            }
            c
        }
    };

    let mut table = Table::new();
    table
        .load_preset(presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            add_style(
                Cell::new(&rust_i18n::t!("planet.name_header")),
                tc_label,
                true,
            ), // "Planet"
            add_style(
                Cell::new(&format!(
                    "{}.{}",
                    rust_i18n::t!("cli.label.gate"),
                    rust_i18n::t!("cli.label.line")
                )),
                tc_label,
                true,
            ),
            add_style(Cell::new(&rust_i18n::t!("cli.label.sign")), tc_label, true),
            add_style(Cell::new(&rust_i18n::t!("cli.label.sign")), tc_label, true),
            add_style(
                Cell::new(&format!(
                    "{}.{}",
                    rust_i18n::t!("cli.label.gate"),
                    rust_i18n::t!("cli.label.line")
                )),
                tc_label,
                true,
            ),
            add_style(
                Cell::new(&rust_i18n::t!("planet.name_header")),
                tc_label,
                true,
            ),
        ]);

    // Set minimum width for Sign columns (index 2 and 3) to prevent squashing
    let min_sign_width = ColumnConstraint::LowerBoundary(comfy_table::Width::Fixed(15));
    if let Some(col) = table.column_mut(2) {
        col.set_constraint(min_sign_width);
    }
    if let Some(col) = table.column_mut(3) {
        col.set_constraint(min_sign_width);
    }

    for (des, pers) in design.iter().zip(personality.iter()) {
        let des_sign = format!("{} {:.2}°", des.zodiac_symbol, des.zodiac_degree);
        let pers_sign = format!("{} {:.2}°", pers.zodiac_symbol, pers.zodiac_degree);

        let des_gate_line = format!("{}.{}", des.gate, des.line);
        let pers_gate_line = format!("{}.{}", pers.gate, pers.line);

        table.add_row(vec![
            add_style(
                Cell::new(&format!("{} {}", des.planet_symbol, des.planet)),
                tc_label,
                false,
            ),
            add_style(Cell::new(&des_gate_line), tc_label, true),
            add_style(Cell::new(&des_sign), tc_label, false),
            add_style(Cell::new(&pers_sign), tc_white, false),
            add_style(Cell::new(&pers_gate_line), tc_white, true),
            add_style(
                Cell::new(&format!("{} {}", pers.planet_symbol, pers.planet)),
                tc_white,
                false,
            ),
        ]);
    }

    writeln!(out, "{}", table).unwrap();

    // Output descriptions (Design + Personality) - only if descriptions exist
    let has_descriptions = personality.iter().any(|p| p.gate_description.is_some());

    if has_descriptions {
        let term_width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80
        };

        // Standardized Headers
        writeln!(
            out,
            "\n{}",
            rust_i18n::t!("cli.section.personality")
                .truecolor(95, 158, 160)
                .bold()
        )
        .unwrap();
        // Removed extra newline here
        write_descriptions(out, personality, term_width);

        writeln!(
            out,
            "\n{}",
            rust_i18n::t!("cli.section.design")
                .truecolor(95, 158, 160)
                .bold()
        )
        .unwrap();
        // Removed extra newline here
        write_descriptions(out, design, term_width);
    }
}

fn write_descriptions(
    out: &mut String,
    data: &[crate::models::PlanetPosition],
    _term_width: usize,
) {
    let desc_color = colored::Color::TrueColor {
        r: 230,
        g: 228,
        b: 208,
    }; // Beige
    let label_color = colored::Color::TrueColor {
        r: 255,
        g: 160,
        b: 122,
    }; // Soft Coral
    let value_color = colored::Color::TrueColor {
        r: 255,
        g: 215,
        b: 0,
    }; // Gold

    for p in data {
        if let (Some(g_desc), Some(l_desc)) = (&p.gate_description, &p.line_description) {
            let gate_hdr_txt = if let Some(g_name) = &p.gate_name {
                format!("{} {}: {}", rust_i18n::t!("cli.label.gate"), p.gate, g_name)
            } else {
                format!("{} {}", rust_i18n::t!("cli.label.gate"), p.gate)
            };

            // Header for Gate
            writeln!(
                out,
                "\n  {} - {}",
                format!("{} {}", p.planet_symbol, p.planet)
                    .color(label_color)
                    .bold(),
                gate_hdr_txt.color(value_color).bold()
            )
            .unwrap();
            write_wrapped(out, g_desc, 4, Some(desc_color), false);

            // Header for Line (Label/Gold/Bold)
            writeln!(
                out,
                "    {}",
                format!("{} {}:", rust_i18n::t!("cli.label.line"), p.line)
                    .color(label_color)
                    .bold()
            )
            .unwrap();
            write_wrapped(out, l_desc, 6, Some(desc_color), false);
        }
    }
}

fn write_gate_section_items(out: &mut String, title: &str, items: &[crate::models::InfoItem]) {
    writeln!(out, "\n{}", title.truecolor(95, 158, 160).bold()).unwrap();
    writeln!(out).unwrap(); // Spacing

    let desc_color = colored::Color::TrueColor {
        r: 230,
        g: 228,
        b: 208,
    }; // Beige
    let label_color = colored::Color::TrueColor {
        r: 255,
        g: 160,
        b: 122,
    }; // Soft Coral
    let value_color = colored::Color::TrueColor {
        r: 255,
        g: 215,
        b: 0,
    }; // Gold

    for item in items {
        if let (Some(planets), Some(gate_id), Some(gate_name)) =
            (&item.planets, item.gate_id, &item.gate_name)
        {
            // New Format: Planet - Gate
            // "☉ Sun, ⊕ Earth - Gate 5: Name"
            let mut planets_vec: Vec<_> = planets.iter().collect();
            planets_vec.sort();

            let planets_str = planets_vec
                .iter()
                .map(|p| format!("{} {}", p.symbol, p.name))
                .collect::<Vec<_>>()
                .join(", ");

            let gate_part = format!(
                "{} {}: {}",
                rust_i18n::t!("cli.label.gate"),
                gate_id,
                gate_name
            );

            writeln!(
                out,
                "  {} - {}",
                planets_str.color(label_color).bold(),
                gate_part.color(value_color).bold()
            )
            .unwrap();
            write_wrapped(out, &item.description, 4, Some(desc_color), false);
        } else {
            // Fallback / Standard InfoItem
            writeln!(out, "  {}", item.label.truecolor(255, 160, 122)).unwrap();
            write_wrapped(out, &item.description, 4, Some(desc_color), false);
        }
    }
}

fn write_wrapped(
    out: &mut String,
    text: &str,
    indent: usize,
    color: Option<colored::Color>,
    dimmed: bool,
) {
    let width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80
    };

    let indent_str = " ".repeat(indent);
    let options = Options::new(width)
        .initial_indent(&indent_str)
        .subsequent_indent(&indent_str);

    let wrapped = textwrap::fill(text, &options);

    let mut style = if let Some(c) = color {
        wrapped.color(c)
    } else {
        wrapped.normal()
    };

    if dimmed {
        style = style.dimmed();
    }

    writeln!(out, "{}", style).unwrap();
}
