mod astro_calc;
mod calc;
mod cli;
mod data;
mod models;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    // Парсинг входных данных
    let (year, month, day) = match cli::parse_date(&args.date) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Ошибка: {}", e);
            std::process::exit(1);
        }
    };

    let (hour, min) = match cli::parse_time(&args.time) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Ошибка: {}", e);
            std::process::exit(1);
        }
    };

    let utc_offset = match cli::parse_utc_offset(&args.utc) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Ошибка: {}", e);
            std::process::exit(1);
        }
    };

    // Расчёт карты
    let chart = calc::build_chart(
        year, month, day, hour, min, utc_offset,
        !args.short, &args.lang,
    );

    // Вывод
    cli::output_chart(&chart, &args.format);
}
