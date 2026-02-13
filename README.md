# Human Design CLI (hd-cli)

A high-performance Rust-based command-line interface for calculating and displaying **Human Design** charts. It uses the `astro-rust` library for precise astronomical calculations and provides a beautiful, colorized terminal output.

## Features

- **Precise Calculations**: Accurate positions for Sun, Earth, Moon, Lunar Nodes, and all planets.
- **Full Chart Analysis**: Calculates Type, Profile, Authority, Strategy, and Incarnation Cross.
- **Detailed Data**: Displays detailed information about Gates (including Sexuality, Fear, Love), Lines, Channels, and Centers.
- **Multiple Output Formats**: Supports interactive Table, JSON, and YAML output.
- **Vibrant Terminal UI**: Features a unified color scheme and responsive layout.
- **Font Awesome Support**: Uses Nerd Fonts for rich zodiac and planet symbols.
- **Multi-language Support**: Descriptions available in English (en), Russian (ru), and Spanish (es). Default is Russian.

## Installation

### Prerequisites

1.  **Rust and Cargo**: Ensure you have [Rust](https://rustup.rs/) installed.
2.  **Nerd Font**: For the best experience with icons, install a [Nerd Font](https://www.nerdfonts.com/) (e.g., *JetBrainsMono Nerd Font*, *FiraCode Nerd Font*) and set it as your terminal font.

### Build from Source

```bash
# Clone the repository
git clone https://github.com/nimblemo/human-design-cli.git
cd human-design-cli

# Build the project
cargo build --release
```

The binary will be available at `./target/release/hd-cli`.

## Usage

Calculate a chart by providing the birth date, time, and UTC offset.

### Basic Command

```bash
hd-cli --date 1990-05-15 --time 14:30 --utc +3
```

### Options

| Flag | Short | Description |
| :--- | :--- | :--- |
| `--date` | `-d` | Birth date in `YYYY-MM-DD` format. |
| `--time` | `-t` | Birth time in `HH:MM` format. |
| `--utc` | `-u` | Timezone offset (e.g., `+3`, `-5`, `+5.5`). |
| `--short` | | Concise output: hides detailed descriptions. |
| `--format` | `-f` | Output format: `table` (default), `json`, `yaml`. |
| `--lang` | `-l` | Language: `ru` (default), `en`, `es`. |
| `--save` | | Save output to file (default filename or custom). |

### Examples

**Concise Table Output:**
```bash
hd-cli -d 1986-05-19 -t 12:00 -u +5 --short
```

**Export to JSON:**
```bash
hd-cli -d 1990-05-15 -t 14:30 -u +3 --format json > chart.json
```

**Save to File:**
```bash
hd-cli -d 1990-05-15 -t 14:30 -u +3 --save my_chart.txt
```

**English Output:**
```bash
hd-cli -d 1990-05-15 -t 14:30 -u +3 --lang en
```

## Project Structure

- `src/main.rs`: Entry point and CLI argument parsing.
- `src/calc.rs`: Core Human Design logic and chart assembly.
- `src/astro_calc.rs`: Astronomical calculations wrapper.
- `src/cli.rs`: Terminal output formatting and UI logic.
- `src/data/`: Data models and database loading.

## Development

The project automatically downloads the necessary gates database during the build process (`build.rs`).

## Testing & Performance

The project includes a comprehensive suite for verification and performance measurement.

### Unit Tests
Run the standard test suite to verify calculations:
```bash
cargo test
```

### Micro-benchmarking
Measure the performance of core calculation functions using `criterion`:
```bash
cargo bench
```

### Load Testing
Measure total throughput and parallel performance (calculates 10,000+ charts):
```bash
cargo run --example load_test --release
```
Typical performance results:
- **Calculation speed**: ~80-120 µs per chart.
- **Throughput**: ~8,000+ charts/sec (multi-threaded).

## License

MIT
