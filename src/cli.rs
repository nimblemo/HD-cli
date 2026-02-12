/// CLI интерфейс: аргументы, форматирование вывода

use clap::{Parser, ValueEnum};
use crate::models::HdChart;
use comfy_table::{Table, Cell, Attribute, Color as TableColor, ContentArrangement, presets};
use colored::*;
use terminal_size::{Width, terminal_size};
use textwrap::Options;

/// Формат вывода
#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    /// Форматированная таблица в терминале
    Table,
    /// JSON формат
    Json,
    /// YAML формат
    Yaml,
}

/// Human Design CLI — расчёт карты Дизайна Человека
///
/// Рассчитывает полную карту Дизайна Человека по дате, времени рождения
/// и часовому поясу (UTC-смещение). Поддерживает вывод в формате таблицы,
/// JSON или YAML. Используйте --short для скрытия детальных описаний.
#[derive(Parser, Debug)]
#[command(name = "hd-cli")]
#[command(version = "0.1.0")]
#[command(about = "Human Design CLI — расчёт карты Дизайна Человека")]
#[command(long_about = "Рассчитывает полную карту HD по дате/времени рождения и UTC-смещению.\n\nПример:\n  hd-cli --date 1990-05-15 --time 14:30 --utc +3\n  hd-cli --date 1990-05-15 --time 14:30 --utc +3 --format json\n  hd-cli --date 1990-05-15 --time 14:30 --utc +3 --short")]
pub struct Cli {
    /// Дата рождения в формате YYYY-MM-DD (например: 1990-05-15)
    #[arg(short = 'd', long)]
    pub date: String,

    /// Время рождения в формате HH:MM (например: 14:30)
    #[arg(short = 't', long)]
    pub time: String,

    /// Часовой пояс как UTC-смещение (например: +3, -5, +5.5)
    #[arg(short = 'u', long)]
    pub utc: String,

    /// Формат вывода: table (по умолчанию), json, yaml
    #[arg(short = 'f', long, default_value = "table")]
    pub format: OutputFormat,

    /// Краткий вывод (скрыть детальные описания ворот, линий, каналов и центров)
    #[arg(long)]
    pub short: bool,

    /// Язык описаний (по умолчанию: ru). Определяет файл данных gates_database_{lang}.json
    #[arg(short = 'l', long, default_value = "ru")]
    pub lang: String,
}

/// Парсинг даты из строки YYYY-MM-DD
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

/// Парсинг времени из строки HH:MM
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

/// Парсинг UTC-смещения из строки (+3, -5, +5.5)
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

/// Вывод карты в указанном формате
pub fn output_chart(chart: &HdChart, format: &OutputFormat) {
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(chart).unwrap();
            println!("{}", json);
        }
        OutputFormat::Yaml => {
            let yaml = serde_yaml::to_string(chart).unwrap();
            println!("{}", yaml);
        }
        OutputFormat::Table => {
            print_table(chart);
        }
    }
}

fn print_table(chart: &HdChart) {
    // Заголовок
    println!("\n{}", "═══════════════════════════════════════════════════════════════".truecolor(95, 158, 160));
    println!("{}", "              HUMAN DESIGN — КАРТА РОЖДЕНИЯ".truecolor(255, 255, 255).bold());
    println!("{}", "═══════════════════════════════════════════════════════════════".truecolor(95, 158, 160));

    // Основная информация
    println!("\n{}", "  ОСНОВНЫЕ ДАННЫЕ".truecolor(95, 158, 160).bold());
    println!(); // Отступ после подзаголовка
    
    let label_color = |s: &str| s.truecolor(255, 160, 122); // Soft Coral (was Gold)
    let value_color = |s: &str| s.truecolor(255, 160, 122); // Soft Coral
    let desc_color = 230_u8; // Beige R
    let desc_color_g = 228_u8; // Beige G
    let desc_color_b = 208_u8; // Beige B

    println!("  {} {} {} UTC{}",
        label_color("Дата:"),
        value_color(&chart.birth_date),
        value_color(&chart.birth_time),
        value_color(&format!("{:+}", chart.utc_offset))
    );
    println!("  {} {}",
        label_color("Тип:"),
        value_color(&chart.hd_type).bold()
    );
    if let Some(ref desc) = chart.type_description {
        print_wrapped(desc, 2, Some(colored::Color::TrueColor { r: desc_color, g: desc_color_g, b: desc_color_b }), false);
    }
    println!("  {} {}",
        label_color("Профиль:"),
        value_color(&chart.profile).bold()
    );
    if let Some(ref desc) = chart.profile_description {
        print_wrapped(desc, 2, Some(colored::Color::TrueColor { r: desc_color, g: desc_color_g, b: desc_color_b }), false);
    }
    println!("  {} {}",
        label_color("Авторитет:"),
        value_color(&chart.authority).bold()
    );
    if let Some(ref desc) = chart.authority_description {
        print_wrapped(desc, 2, Some(colored::Color::TrueColor { r: desc_color, g: desc_color_g, b: desc_color_b }), false);
    }
    println!("  {} {}",
        label_color("Стратегия:"),
        value_color(&chart.strategy).bold()
    );
    if let Some(ref desc) = chart.strategy_description {
        print_wrapped(desc, 2, Some(colored::Color::TrueColor { r: desc_color, g: desc_color_g, b: desc_color_b }), false);
    }
    println!("  {} {}", label_color("Инкарнационный крест:"), value_color(&chart.incarnation_cross).bold());
    if let Some(ref desc) = chart.cross_description {
        print_wrapped(desc, 2, Some(colored::Color::TrueColor { r: desc_color, g: desc_color_g, b: desc_color_b }), false);
    }

    // Бизнес (перемещен сюда)
    if let Some(ref biz) = chart.business {
        println!("\n{}", "  БИЗНЕС".truecolor(95, 158, 160).bold());
        println!(); // Отступ

        for b in biz {
            println!("  {} {}:",
                format!("Ворота {}:", b.gate).truecolor(255, 160, 122),
                b.title.truecolor(255, 160, 122)
            );
            if let Some(ref desc) = b.description {
                print_wrapped(desc, 4, Some(colored::Color::TrueColor { r: desc_color, g: desc_color_g, b: desc_color_b }), false);
            }
        }
    }

    // 4. Планеты (Общая таблица)
    print_combined_planet_table(&chart.design, &chart.personality);

    // 5. Каналы
    if !chart.channels.is_empty() {
        println!("\n{}", "  КАНАЛЫ".truecolor(95, 158, 160).bold());
        println!(); // Отступ
        
        let has_descriptions = chart.channels.iter().any(|ch| ch.description.is_some());

        let mut table = Table::new();
        table
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);
        
        let mut headers = vec![
            Cell::new("Канал").add_attribute(Attribute::Bold).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }),
            Cell::new("Название").add_attribute(Attribute::Bold).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }),
        ];
        if has_descriptions {
            headers.push(Cell::new("Описание").add_attribute(Attribute::Bold).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }));
        }
        table.set_header(headers);

        for ch in &chart.channels {
             let mut row = vec![
                Cell::new(&ch.key).fg(TableColor::Rgb { r: 95, g: 158, b: 160 }),
                Cell::new(&ch.name).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }).add_attribute(Attribute::Bold),
             ];
             if has_descriptions {
                let desc = ch.description.clone().unwrap_or_default();
                row.push(Cell::new(&desc).fg(TableColor::Rgb { r: desc_color, g: desc_color_g, b: desc_color_b }));
             }
             table.add_row(row);
        }
        println!("{}", table);
    }

    // Центры
    println!("\n{}", "  ЦЕНТРЫ".truecolor(95, 158, 160).bold());
    println!(); // Отступ

    let mut table = Table::new();
    table
        .load_preset(presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Центр").add_attribute(Attribute::Bold).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }),
        Cell::new("Описание / Статус").add_attribute(Attribute::Bold).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }),
    ]);
    for center in &chart.centers {
        let status = if center.defined { "● Определён" } else { "○ Открыт" };
        let height_color = if center.defined { TableColor::Rgb { r: 255, g: 160, b: 122 } } else { TableColor::DarkGrey };
        let color = height_color;
        
        // Если есть поведение (full mode), показываем его. Иначе статус.
        let content = if let Some(ref beh) = center.behavior {
            beh.clone()
        } else {
            status.to_string()
        };

        table.add_row(vec![
            Cell::new(&center.name).fg(color).add_attribute(Attribute::Bold),
            Cell::new(&content).fg(TableColor::Rgb { r: desc_color, g: desc_color_g, b: desc_color_b }),
        ]);
    }
    println!("{}", table);

    // Дополнительная информация - показываем, если есть хоть что-то
    let has_extra = chart.motivation.is_some() || chart.environment.is_some()
        || chart.diet.is_some() || chart.fear.is_some()
        || chart.sexuality.is_some() || chart.love.is_some() || chart.vision.is_some();
    
    // Показываем блок дополнительно только если включен режим --full, 
    // ТАК КАК в calc.rs мы теперь всегда вычисляем эти поля. 
    // Если пользователь хочет видеть их всегда, уберем проверку.
    // Но обычно в CLI "Additional info" скрывают.
    // Однако, задача "Add name" к Motivation/Diet подразумевает, что пользователь хочет это видеть.
    // Ранее условие было `has_extra` (которое зависело от того, вернул ли calc что-то).
    // Теперь calc возвращает всегда (если в БД есть).
    // Оставим показ ВНЕ зависимости от full, но только если поля есть.
    // НО layout user request: "Additional Info Block: Conditionally hidden unless the --full flag is present". 
    // Wait, the previous session summary said "Conditionally hidden unless the --full flag is present".
    // I should check `if chart.motivation.is_some() && has_extra`.

    // NOTE: In `calc.rs` I removed `if full` check for motivation/diet calculation.
    // So `has_extra` will be true if DB matches.
    // If I want to hide it, I should check `full` here? 
    // Or did I misuse `full` in `calc.rs`? 
    // In `calc.rs` I passed `full` to `build_chart`, but only used it for some descriptions.
    // 
    // Decision: Show it always if available, OR restrict to Full?
    // Summary said: "Additional Info Block: Conditionally hidden unless the --full flag is present".
    // I will respect that constraint from previous summary.
    // But how do I know if `full` is enabled here in `print_table`?
    // `print_table` takes `&HdChart`. `HdChart` doesn't store `full` flag status explicitly (only implicitly via presence of optional fields).
    // BUT `motivaton` is now always present (if db has it).
    //
    // I'll check `chart.strategy_description` presence as a proxy for `full` mode? 
    // Or just print it. The user explicitly asked to REFINE this.
    // "Additional Info Block: Conditionally hidden unless the --full flag is present" -> this means I should check something.
    // `type_description` is only set if `full` is true.
    
    let is_full_mode = chart.type_description.is_some();

    if has_extra && is_full_mode {
        println!("\n{}", "  ДОПОЛНИТЕЛЬНО".truecolor(95, 158, 160).bold());
        println!(); // Отступ

        if let Some(ref m) = chart.motivation {
            print_info_items("Мотивация:", m);
        }
        if let Some(ref v) = chart.vision {
            print_info_items("Видение:", v);
        }
        if let Some(ref e) = chart.environment {
             print_info_items("Среда:", e);
        }
        if let Some(ref d) = chart.diet {
             print_info_items("Диета:", d);
        }
        if let Some(ref f) = chart.fear {
             print_kv_wrapped("Страх:", f);
        }
        if let Some(ref s) = chart.sexuality {
             print_kv_wrapped("Сексуальность:", s);
        }
        if let Some(ref l) = chart.love {
             print_kv_wrapped("Любовь:", l);
        }
    }


}

fn print_info_items(title: &str, items: &[crate::models::InfoItem]) {
    println!("  {}", title.truecolor(255, 160, 122)); // Soft Coral (was Gold)
    
    let label_color = colored::Color::TrueColor { r: 255, g: 160, b: 122 };
    let desc_color = colored::Color::TrueColor { r: 230, g: 228, b: 208 };

    for item in items {
        println!("    {}", item.label.color(label_color));
        if !item.description.is_empty() {
            print_wrapped(&item.description, 6, Some(desc_color), false);
        }
    }
}


fn print_combined_planet_table(design: &[crate::models::PlanetPosition], personality: &[crate::models::PlanetPosition]) {
    println!("\n{}", "  ПЛАНЕТЫ (Planets)".truecolor(95, 158, 160).bold());
    println!(); // Отступ

    let mut table = Table::new();
    table
        .load_preset(presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Планета").add_attribute(Attribute::Bold).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }),
            Cell::new("Ворота.Линия").add_attribute(Attribute::Bold).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }),
            Cell::new("Знак").add_attribute(Attribute::Bold).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }),
            Cell::new("Знак").add_attribute(Attribute::Bold).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }),
            Cell::new("Ворота.Линия").add_attribute(Attribute::Bold).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }),
            Cell::new("Планета").add_attribute(Attribute::Bold).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }),
        ]);

    
    for (des, pers) in design.iter().zip(personality.iter()) {
        let des_sign = format!("{} {:.2}°", des.zodiac_symbol, des.zodiac_degree);
        let pers_sign = format!("{} {:.2}°", pers.zodiac_symbol, pers.zodiac_degree);
        
        let des_gate_line = format!("{}.{}", des.gate, des.line);
        let pers_gate_line = format!("{}.{}", pers.gate, pers.line);

        table.add_row(vec![
            Cell::new(&des.planet).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }), // Soft Coral (was Coral)
            Cell::new(&des_gate_line).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }).add_attribute(Attribute::Bold),
            Cell::new(&des_sign).fg(TableColor::Rgb { r: 255, g: 160, b: 122 }),
            Cell::new(&pers_sign).fg(TableColor::White), // Personality in White for visibility
            Cell::new(&pers_gate_line).fg(TableColor::White).add_attribute(Attribute::Bold),
            Cell::new(&pers.planet).fg(TableColor::White),
        ]);
    }

    println!("{}", table);

    // Вывод описаний (Дизайн + Личность) - только если есть описания
    let has_descriptions = personality.iter().any(|p| p.gate_description.is_some());
    
    if has_descriptions {
        let term_width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80
        };

        // Standardized Headers
        println!("\n{}", "  ОПИСАНИЯ ЛИЧНОСТИ (Personality)".truecolor(95, 158, 160).bold());
        println!(); // Отступ
        print_descriptions(personality, term_width);

        println!("\n{}", "  ОПИСАНИЯ ДИЗАЙНА (Design)".truecolor(95, 158, 160).bold());
        println!(); // Отступ
        print_descriptions(design, term_width);
    }
}

fn print_descriptions(data: &[crate::models::PlanetPosition], _term_width: usize) {
    let desc_color = colored::Color::TrueColor { r: 230, g: 228, b: 208 }; // Beige
    let label_color = colored::Color::TrueColor { r: 255, g: 160, b: 122 }; // Soft Coral (was Gold)
    let value_color = colored::Color::TrueColor { r: 255, g: 160, b: 122 }; // Soft Coral

    for p in data {
        if let (Some(g_desc), Some(l_desc)) = (&p.gate_description, &p.line_description) {
            let gate_info = if let Some(g_name) = &p.gate_name {
                 format!("Ворота {}: {}", p.gate, g_name)
            } else {
                 format!("Ворота {}", p.gate)
            };
            
            // Header for Gate
            // Planet (Label/Gold/Bold) - Gate Info (Value/Coral/Bold)
            println!("\n  {} - {}", p.planet.color(label_color).bold(), gate_info.color(value_color).bold());
            print_wrapped(g_desc, 4, Some(desc_color), false);

            // Header for Line (Label/Gold/Bold)
            println!("    {}", format!("Линия {}:", p.line).color(label_color).bold());
            print_wrapped(l_desc, 6, Some(desc_color), false);
        }
    }
}

fn print_wrapped(text: &str, indent: usize, color: Option<colored::Color>, dimmed: bool) {
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
    
    println!("{}", style);
}

fn print_kv_wrapped(key: &str, value: &str) {
    println!("  {}", key.truecolor(255, 160, 122)); // Soft Coral (was Gold)
    print_wrapped(value, 4, Some(colored::Color::TrueColor { r: 230, g: 228, b: 208 }), false);
}
