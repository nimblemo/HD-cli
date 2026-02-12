use std::fs;
use std::path::Path;
use std::process::Command;

const GITHUB_RAW_URL: &str =
    "https://raw.githubusercontent.com/nimblemo/hd-parser/refs/heads/master/data/gates_database.json";

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let data_dir = Path::new(&manifest_dir).join("data");
    let dest = data_dir.join("gates_database.json");

    // Ensure the data directory exists
    fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    if !dest.exists() {
        println!("cargo:warning=gates_database.json not found. Downloading...");
        
        // Use curl to download — available on Windows 10+, macOS, Linux
        let result = Command::new("curl")
            .args([
                "-fsSL",                           // fail silently, follow redirects
                "--connect-timeout", "15",         // 15s connection timeout
                "--max-time", "60",                // 60s total timeout
                "-o", dest.to_str().expect("Path transition failed"),
                GITHUB_RAW_URL,
            ])
            .status();

        match result {
            Ok(status) if status.success() => {
                let metadata = fs::metadata(&dest).expect("Failed to get metadata");
                if metadata.len() > 0 {
                    println!("cargo:warning=Downloaded gates_database.json from GitHub ✓ ({} bytes)", metadata.len());
                } else {
                    panic!("Downloaded gates_database.json is empty!");
                }
            }
            Ok(status) => {
                panic!("curl failed with exit code {}. Check your internet connection.", status.code().unwrap_or(-1));
            }
            Err(e) => {
                panic!("Could not run curl: {e}. Please install curl or download the file manually to data/gates_database.json");
            }
        }
    } else {
        println!("cargo:warning=Using existing gates_database.json at {:?}", dest);
    }

    // Re-run build.rs if build.rs or the data file changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", dest.display());
}
