use std::fs;
use std::path::Path;
use std::process::Command;

const GITHUB_RAW_BASE: &str = "https://raw.githubusercontent.com/nimblemo/hd-parser/refs/heads/master/data/";
const FILES: &[&str] = &[
    "gates_database_ru.json",
    "gates_database_en.json",
    "gates_database_es.json",
];

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let data_dir = Path::new(&manifest_dir).join("data");

    // Ensure the data directory exists
    fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    for file_name in FILES {
        let url = format!("{}{}", GITHUB_RAW_BASE, file_name);
        let dest = data_dir.join(file_name);

        if !dest.exists() {
            println!("cargo:warning={} not found. Downloading...", file_name);
            
            // Use curl to download — available on Windows 10+, macOS, Linux
            let result = Command::new("curl")
                .args([
                    "-fsSL",                           // fail silently, follow redirects
                    "--connect-timeout", "15",         // 15s connection timeout
                    "--max-time", "60",                // 60s total timeout
                    "-o", dest.to_str().expect("Path transition failed"),
                    &url,
                ])
                .status();

            match result {
                Ok(status) if status.success() => {
                    let metadata = fs::metadata(&dest).expect("Failed to get metadata");
                    if metadata.len() > 0 {
                        println!("cargo:warning=Downloaded {} from GitHub ✓ ({} bytes)", file_name, metadata.len());
                    } else {
                        // Clean up empty file
                        let _ = fs::remove_file(&dest);
                        panic!("Downloaded {} is empty!", file_name);
                    }
                }
                Ok(status) => {
                    panic!("curl failed for {} with exit code {}. Check your internet connection.", file_name, status.code().unwrap_or(-1));
                }
                Err(e) => {
                    panic!("Could not run curl for {}: {e}. Please install curl or download the file manually to data/{}", file_name, file_name);
                }
            }
        } else {
            println!("cargo:warning=Using existing {} at {:?}", file_name, dest);
        }
        println!("cargo:rerun-if-changed={}", dest.display());
    }

    // Re-run build.rs if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
}
