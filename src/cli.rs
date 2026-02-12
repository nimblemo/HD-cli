/// CLI interface: arguments, output formatting

use clap::{Parser, ValueEnum};
use crate::models::HdChart;
use comfy_table::{Table, Cell, Attribute, Color as TableColor, ContentArrangement, presets};
use colored::*;
use terminal_size::{Width, terminal_size};
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

/// Human Design CLI — Human Design chart calculation
///
/// Calculates full Human Design chart by date, time of birth
/// and time zone (UTC offset). Supports output in table,
/// JSON or YAML format. Use --short to hide detailed descriptions.
#[derive(Parser, Debug)]
#[command(name = "hd-cli")]
#[command(version = "0.1.0")]
#[command(about = "Human Design CLI — расчёт карты Дизайна Человека")]
#[command(long_about = "Рассчитывает полную карту HD по дате/времени рождения и UTC-смещению.\n\nПример:\n  hd-cli --date 1990-05-15 --time 14:30 --utc +3\n  hd-cli --date 1990-05-15 --time 14:30 --utc +3 --format json\n  hd-cli --date 1990-05-15 --time 14:30 --utc +3 --short")]
pub struct Cli {
    /// Date of birth in YYYY-MM-DD format (e.g. 1990-05-15)
    #[arg(short = 'd', long)]
    pub date: String,

    /// Time of birth in HH:MM format (e.g. 14:30)
    #[arg(short = 't', long)]
    pub time: String,

    /// Time zone as UTC offset (e.g. +3, -5, +5.5)
    #[arg(short = 'u', long)]
    pub utc: String,

    /// Output format: table (default), json, yaml
    #[arg(short = 'f', long, default_value = "table")]
    pub format: OutputFormat,

    /// Short output (hide detailed descriptions of gates, lines, channels and centers)
    #[arg(long)]
    pub short: bool,

    /// Description language (default: ru). Determines data file gates_database_{lang}.json
    #[arg(short = 'l', long, default_value = "ru")]
    pub lang: String,

    /// Save output to file. If filename is not specified, it will be generated automatically.
    #[arg(long, num_args(0..=1), default_missing_value = "default")]
    pub save: Option<String>,
}

/// Parse date from YYYY-MM-DD string
pub fn parse_date(s: &str) -> Result<(i32, u8, u8), String> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return Err(format!("Некорректный формат даты: '{}'. Ожидается YYYY-MM-DD", s));
    }
    let year: i32 = parts[0].parse().map_err(|_| format!("Некорректный год: '{}'", parts[0]))?;
    let month: u8 = parts[1].parse().map_err(|_| format!("Некорректный месяц: '{}'", parts[1]))?;
    let day: u8 = parts[2].parse().map_err(|_| format!("Некорректный день: '{}'", parts[2]))?;

    if month < 1 || month > 12 {
        return Err(format!("Месяц должен быть от 1 до 12, получено: {}", month));
    }
    if day < 1 || day > 31 {
        return Err(format!("День должен быть от 1 до 31, получено: {}", day));
    }
    Ok((year, month, day))
}

/// Parse time from HH:MM string
pub fn parse_time(s: &str) -> Result<(u8, u8), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(format!("Некорректный формат времени: '{}'. Ожидается HH:MM", s));
    }
    let hour: u8 = parts[0].parse().map_err(|_| format!("Некорректный час: '{}'", parts[0]))?;
    let min: u8 = parts[1].parse().map_err(|_| format!("Некорректные минуты: '{}'", parts[1]))?;

    if hour > 23 {
        return Err(format!("Час должен быть от 0 до 23, получено: {}", hour));
    }
    if min > 59 {
        return Err(format!("Минуты должны быть от 0 до 59, получено: {}", min));
    }
    Ok((hour, min))
}

/// Parse UTC offset from string (+3, -5, +5.5)
pub fn parse_utc_offset(s: &str) -> Result<f64, String> {
    let s = s.trim();
    let offset: f64 = s.parse().map_err(|_| {
        format!("Некорректное UTC-смещение: '{}'. Ожидается число, например: +3, -5, +5.5", s)
    })?;
    if offset < -12.0 || offset > 14.0 {
        return Err(format!("UTC-смещение должно быть от -12 до +14, получено: {}", offset));
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
    writeln!(out, "\n{}", "═══════════════════════════════════════════════════════════════".truecolor(95, 158, 160)).unwrap();
    writeln!(out, "{}", "              HUMAN DESIGN — КАРТА РОЖДЕНИЯ".truecolor(255, 255, 255).bold()).unwrap();
    writeln!(out, "{}", "═══════════════════════════════════════════════════════════════".truecolor(95, 158, 160)).unwrap();

    // Main information
    writeln!(out, "\n{}", "  ОСНОВНЫЕ ДАННЫЕ".truecolor(95, 158, 160).bold()).unwrap();
    writeln!(out).unwrap(); // Spacing
    
    let label_color = |s: &str| s.truecolor(255, 160, 122); // Soft Coral
    let value_color = |s: &str| s.truecolor(255, 215, 0); // Gold
    let desc_color = colored::Color::TrueColor { r: 230, g: 228, b: 208 }; // Beige

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


    writeln!(out, "  {} {} {} UTC{}",
        label_color("Дата:"),
        value_color(&chart.birth_date),
        value_color(&chart.birth_time),
        value_color(&format!("{:+}", chart.utc_offset))
    ).unwrap();
    writeln!(out).unwrap(); // Empty line after Date for spacing


    writeln!(out, "  {} {}",
        label_color("Тип:"),
        value_color(&chart.hd_type).bold()
    ).unwrap();
    if let Some(ref desc) = chart.type_description {
        write_wrapped(&mut out, desc, 4, Some(desc_color), false);
    }
    writeln!(out).unwrap(); // Empty line after item


    writeln!(out, "  {} {}",
        label_color("Профиль:"),
        value_color(&chart.profile).bold()
    ).unwrap();
    if let Some(ref desc) = chart.profile_description {
        write_wrapped(&mut out, desc, 4, Some(desc_color), false);
    }
    writeln!(out).unwrap(); // Empty line after item


    writeln!(out, "  {} {}",
        label_color("Авторитет:"),
        value_color(&chart.authority).bold()
    ).unwrap();
    if let Some(ref desc) = chart.authority_description {
        write_wrapped(&mut out, desc, 4, Some(desc_color), false);
    }
    writeln!(out).unwrap(); // Empty line after item


    writeln!(out, "  {} {}",
        label_color("Стратегия:"),
        value_color(&chart.strategy).bold()
    ).unwrap();
    if let Some(ref desc) = chart.strategy_description {
        write_wrapped(&mut out, desc, 4, Some(desc_color), false);
    }
    writeln!(out).unwrap(); // Empty line after item


    writeln!(out, "  {} {}", label_color("Инкарнационный крест:"), value_color(&chart.incarnation_cross).bold()).unwrap();
    if let Some(ref desc) = chart.cross_description {
        write_wrapped(&mut out, desc, 4, Some(desc_color), false);
    }
    writeln!(out).unwrap(); // Empty line after item


    // Business
    if let Some(ref biz) = chart.business {
        writeln!(out, "\n{}", "  БИЗНЕС".truecolor(95, 158, 160).bold()).unwrap();
        writeln!(out).unwrap(); // Spacing

        for b in biz {
            writeln!(out, "  {} {}:",
                format!("Ворота {}:", b.gate).truecolor(255, 160, 122), // Coral
                b.title.truecolor(255, 215, 0) // Gold
            ).unwrap();
            if let Some(ref desc) = b.description {
                write_wrapped(&mut out, desc, 4, Some(desc_color), false);
            }
        }
    }

    // 4. CHANNELS (Moved here, after Business)
    if !chart.channels.is_empty() {
        writeln!(out, "\n{}", "  КАНАЛЫ".truecolor(95, 158, 160).bold()).unwrap();
        writeln!(out).unwrap(); // Отступ
        
        let has_descriptions = chart.channels.iter().any(|ch| ch.description.is_some());

        let mut table = Table::new();
        table
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);
        
        let mut headers = vec![
            add_style(Cell::new("Канал"), TableColor::Rgb { r: 255, g: 160, b: 122 }, true), // Coral
            add_style(Cell::new("Название"), TableColor::Rgb { r: 255, g: 215, b: 0 }, true), // Gold
        ];
        if has_descriptions {
            headers.push(add_style(Cell::new("Описание"), TableColor::Rgb { r: 255, g: 160, b: 122 }, true)); // Coral
        }
        table.set_header(headers);

        for ch in &chart.channels {
             let mut row = vec![
                add_style(Cell::new(&ch.key), TableColor::Rgb { r: 95, g: 158, b: 160 }, false),
                add_style(Cell::new(&ch.name), TableColor::Rgb { r: 255, g: 215, b: 0 }, true), // Gold
             ];
             if has_descriptions {
                let desc = ch.description.clone().unwrap_or_default();
                row.push(add_style(Cell::new(&desc), TableColor::Rgb { r: 230, g: 228, b: 208 }, false));
             }
             table.add_row(row);
        }
        writeln!(out, "{}", table).unwrap();
    }

    // 5. Planets (General table) (Now here)
    write_combined_planet_table(&mut out, &chart.design, &chart.personality, plain);

    // Centers
    writeln!(out, "\n{}", "  ЦЕНТРЫ".truecolor(95, 158, 160).bold()).unwrap();
    writeln!(out).unwrap(); // Spacing

    let mut table = Table::new();
    table
        .load_preset(presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        add_style(Cell::new("Центр"), TableColor::Rgb { r: 255, g: 160, b: 122 }, true),
        add_style(Cell::new("Описание / Статус"), TableColor::Rgb { r: 255, g: 160, b: 122 }, true),
    ]);
    for center in &chart.centers {
        let status = if center.defined { "● Определён" } else { "○ Открыт" };
        let height_color = if center.defined { TableColor::Rgb { r: 255, g: 215, b: 0 } } else { TableColor::DarkGrey }; // Gold for defined
        let color = height_color;
        
        let content = if let Some(ref beh) = center.behavior {
            beh.clone()
        } else {
            status.to_string()
        };

        table.add_row(vec![
            add_style(Cell::new(&center.name), color, true),
            add_style(Cell::new(&content), TableColor::Rgb { r: 230, g: 228, b: 208 }, false),
        ]);
    }
    writeln!(out, "{}", table).unwrap();

    // Additional information
    let has_extra = chart.motivation.is_some() || chart.environment.is_some()
        || chart.diet.is_some() || chart.fear.is_some()
        || chart.sexuality.is_some() || chart.love.is_some() || chart.vision.is_some();
    
    let is_full_mode = chart.type_description.is_some(); 

    if has_extra && is_full_mode {
        writeln!(out, "\n{}", "  ДОПОЛНИТЕЛЬНО".truecolor(95, 158, 160).bold()).unwrap();
        writeln!(out).unwrap(); // Spacing

        if let Some(ref m) = chart.motivation {
            write_info_items(&mut out, "Мотивация:", m);
        }
        if let Some(ref v) = chart.vision {
            write_info_items(&mut out, "Видение:", v);
        }
        if let Some(ref e) = chart.environment {
             write_info_items(&mut out, "Среда:", e);
        }
        if let Some(ref d) = chart.diet {
             write_info_items(&mut out, "Диета:", d);
        }
        if let Some(ref f) = chart.fear {
             write_kv_wrapped(&mut out, "Страх:", f);
        }
        if let Some(ref s) = chart.sexuality {
             write_kv_wrapped(&mut out, "Сексуальность:", s);
        }
        if let Some(ref l) = chart.love {
             write_kv_wrapped(&mut out, "Любовь:", l);
        }
    }

    out
}

fn write_info_items(out: &mut String, title: &str, items: &[crate::models::InfoItem]) {
    writeln!(out, "  {}", title.truecolor(255, 215, 0)).unwrap(); // Gold Title
    
    let label_color = colored::Color::TrueColor { r: 255, g: 160, b: 122 };
    let desc_color = colored::Color::TrueColor { r: 230, g: 228, b: 208 };

    for item in items {
        writeln!(out, "    {}", item.label.color(label_color)).unwrap();
        if !item.description.is_empty() {
            write_wrapped(out, &item.description, 6, Some(desc_color), false);
        }
    }
}


fn write_combined_planet_table(out: &mut String, design: &[crate::models::PlanetPosition], personality: &[crate::models::PlanetPosition], plain: bool) {
    writeln!(out, "\n{}", "  ПЛАНЕТЫ (Planets)".truecolor(95, 158, 160).bold()).unwrap();
    // Removed extra newline here


    let tc_label = TableColor::Rgb { r: 255, g: 160, b: 122 };
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
            add_style(Cell::new("Планета"), tc_label, true),
            add_style(Cell::new("Ворота.Линия"), tc_label, true),
            add_style(Cell::new("Знак"), tc_label, true),
            add_style(Cell::new("Знак"), tc_label, true),
            add_style(Cell::new("Ворота.Линия"), tc_label, true),
            add_style(Cell::new("Планета"), tc_label, true),
        ]);

    
    for (des, pers) in design.iter().zip(personality.iter()) {
        let des_sign = format!("{} {:.2}°", des.zodiac_symbol, des.zodiac_degree);
        let pers_sign = format!("{} {:.2}°", pers.zodiac_symbol, pers.zodiac_degree);
        
        let des_gate_line = format!("{}.{}", des.gate, des.line);
        let pers_gate_line = format!("{}.{}", pers.gate, pers.line);

        table.add_row(vec![
            add_style(Cell::new(&des.planet), tc_label, false),
            add_style(Cell::new(&des_gate_line), tc_label, true),
            add_style(Cell::new(&des_sign), tc_label, false),
            add_style(Cell::new(&pers_sign), tc_white, false),
            add_style(Cell::new(&pers_gate_line), tc_white, true),
            add_style(Cell::new(&pers.planet), tc_white, false),
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
        writeln!(out, "\n{}", "  ОПИСАНИЯ ЛИЧНОСТИ (Personality)".truecolor(95, 158, 160).bold()).unwrap();
        // Removed extra newline here
        write_descriptions(out, personality, term_width);

        writeln!(out, "\n{}", "  ОПИСАНИЯ ДИЗАЙНА (Design)".truecolor(95, 158, 160).bold()).unwrap();
        // Removed extra newline here
        write_descriptions(out, design, term_width);
    }
}

fn write_descriptions(out: &mut String, data: &[crate::models::PlanetPosition], _term_width: usize) {
    let desc_color = colored::Color::TrueColor { r: 230, g: 228, b: 208 }; // Beige
    let label_color = colored::Color::TrueColor { r: 255, g: 160, b: 122 }; // Soft Coral
    let value_color = colored::Color::TrueColor { r: 255, g: 215, b: 0 }; // Gold

    for p in data {
        if let (Some(g_desc), Some(l_desc)) = (&p.gate_description, &p.line_description) {
            let gate_info = if let Some(g_name) = &p.gate_name {
                 format!("Ворота {}: {}", p.gate, g_name)
            } else {
                 format!("Ворота {}", p.gate)
            };
            
            // Header for Gate
            writeln!(out, "\n  {} - {}", p.planet.color(label_color).bold(), gate_info.color(value_color).bold()).unwrap();
            write_wrapped(out, g_desc, 4, Some(desc_color), false);

            // Header for Line (Label/Gold/Bold)
            writeln!(out, "    {}", format!("Линия {}:", p.line).color(label_color).bold()).unwrap();
            write_wrapped(out, l_desc, 6, Some(desc_color), false);
        }
    }
}

fn write_wrapped(out: &mut String, text: &str, indent: usize, color: Option<colored::Color>, dimmed: bool) {
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

fn write_kv_wrapped(out: &mut String, key: &str, value: &str) {
    writeln!(out, "  {}", key.truecolor(255, 160, 122)).unwrap(); // Soft Coral
    write_wrapped(out, value, 4, Some(colored::Color::TrueColor { r: 230, g: 228, b: 208 }), false);
}
