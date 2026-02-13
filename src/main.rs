use clap::Parser;
use hd_cli::cli::{self, Cli, Commands};
use hd_cli::calc;
use hd_cli::config::Config;

// Init translations
rust_i18n::i18n!("locales");

fn main() {
    let args = Cli::parse();
    
    // 1. Load configuration
    let mut config = Config::load();

    // 2. Handle subcommands
    if let Some(command) = args.command {
        match command {
            Commands::Config { set_lang } => {
                if let Some(lang) = set_lang {
                    match config.set_language(&lang) {
                        Ok(_) => println!("Default language set to '{}'", lang),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                } else {
                    println!("Current default language: {}", config.language);
                }
                return; // Exit after handling config
            }
        }
    }

    // 3. Determine language
    // Priority: CLI arg > Config > Default (built into Config)
    let lang = args.lang.unwrap_or(config.language);
    rust_i18n::set_locale(&lang);

    // 4. Validate required arguments for calculation
    // Since we made them Option to support subcommands, we must check them here.
    if args.date.is_none() || args.time.is_none() || args.utc.is_none() {
        // If not running a subcommand and missing args, print help
        use clap::CommandFactory;
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();
        std::process::exit(1);
    }

    let date_str = args.date.unwrap();
    let time_str = args.time.unwrap();
    let utc_str = args.utc.unwrap();

    // Parse input data
    let (year, month, day) = match cli::parse_date(&date_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let (hour, min) = match cli::parse_time(&time_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let utc_offset = match cli::parse_utc_offset(&utc_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Calculate chart
    // We pass the resolved `lang` to calc::build_chart so it can pick the right DB
    // Note: rust_i18n::set_locale affects translations (t! macro),
    // but the database content is retrieved via getting the right DB instance.
    let chart = calc::build_chart(
        year, month, day, hour, min, utc_offset,
        !args.short, &lang,
    );

    // 1. Console output (with colors)
    let output = cli::generate_output(&chart, &args.format, false);
    println!("{}", output);

    // 2. Save to file (if flag is specified)
    if let Some(ref save_val) = args.save {
        // Generate again without colors (plain=true)
        let file_output = cli::generate_output(&chart, &args.format, true);

        let filename = if save_val == "default" {
            format!("hd_chart_{}_{}.txt", date_str, time_str.replace(':', "-"))
        } else {
            save_val.clone()
        };

        match std::fs::write(&filename, file_output) {
            Ok(_) => println!("\n{}", rust_i18n::t!("error.save_file", filename = filename)),
            Err(e) => eprintln!("\n{}", rust_i18n::t!("error.save_error", error = e.to_string())),
        }
    }
}
