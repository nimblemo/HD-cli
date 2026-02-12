use hd_cli::calc::build_chart;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    println!("Starting parallel load test (10,000 chart calculations)...");

    let count = 10_000;
    let start = Instant::now();

    // Parallel processing with Rayon
    let results: Vec<_> = (0..count)
        .into_par_iter()
        .map(|_| {
            build_chart(
                1990, 5, 15, 14, 30, 3.0,
                false, // short mode (faster)
                "ru",
            )
        })
        .collect();

    let duration = start.elapsed();
    let charts_per_sec = count as f64 / duration.as_secs_f64();

    println!("--------------------------------------------------");
    println!("Processed {} charts in {:?}", results.len(), duration);
    println!("Average time per chart: {:?}", duration / count as u32);
    println!("Throughput: {:.2} charts/sec", charts_per_sec);
    println!("--------------------------------------------------");
}
