use clap::Parser;
use hd_cli::cli::{self, Cli};
use hd_cli::calc;

fn main() {
    let args = Cli::parse();

    // Parse input data
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

    // Calculate chart
    let chart = calc::build_chart(
        year, month, day, hour, min, utc_offset,
        !args.short, &args.lang,
    );

    // 1. Console output (with colors)
    let output = cli::generate_output(&chart, &args.format, false);
    println!("{}", output);

    // 2. Save to file (if flag is specified)
    if let Some(ref save_val) = args.save {
        // Generate again without colors (plain=true)
        let file_output = cli::generate_output(&chart, &args.format, true);

        let filename = if save_val == "default" {
            format!("hd_chart_{}_{}.txt", args.date, args.time.replace(':', "-"))
        } else {
            save_val.clone()
        };

        match std::fs::write(&filename, file_output) {
            Ok(_) => println!("\nРезультат сохранён в файл: {}", filename),
            Err(e) => eprintln!("\nОшибка при сохранении файла: {}", e),
        }
    }
}
