//! Benchmark for parallel encoding threshold tuning.
//!
//! The default 50KB threshold is tuned for typical systems. Run this to find
//! the optimal value for your specific hardware:
//!
//! ```sh
//! cargo run --release --example parallel_benchmark
//! ```
//!
//! Look for the "crossover" point where parallel speedup exceeds 1.1x.

use std::time::Instant;
use tiktoken_rs::{o200k_base, CoreBPE};

const WARMUP_ITERATIONS: usize = 3;

fn benchmark(bpe: &CoreBPE, text: &str, iterations: usize) -> (f64, f64, usize) {
    // Warmup to stabilize CPU frequency and caches
    for _ in 0..WARMUP_ITERATIONS {
        let _ = bpe.encode_ordinary(text);
        let _ = bpe.encode_ordinary_parallel(text);
    }

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = bpe.encode_ordinary(text);
    }
    let single = start.elapsed().as_secs_f64() / iterations as f64;

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = bpe.encode_ordinary_parallel(text);
    }
    let parallel = start.elapsed().as_secs_f64() / iterations as f64;

    (single, parallel, bpe.encode_ordinary(text).len())
}

fn main() {
    let bpe = o200k_base().expect("failed to load o200k_base");
    let num_threads = rayon::current_num_threads();

    println!("Parallel Encoding Threshold Benchmark");
    println!("======================================");
    println!("CPU threads: {}", num_threads);
    println!();

    let base = "The quick brown fox jumps over the lazy dog.\n";

    // Test across size range
    println!(
        "{:>8} {:>8} {:>12} {:>12} {:>8}",
        "Size", "Tokens", "Single", "Parallel", "Speedup"
    );
    println!("{}", "-".repeat(52));

    let sizes_kb = [10, 20, 30, 40, 50, 60, 80, 100, 150, 200, 500];
    let mut crossover = None;

    for kb in sizes_kb {
        let text = base.repeat((kb * 1000) / base.len());
        let iters = if kb < 100 { 50 } else { 20 };
        let (single, parallel, tokens) = benchmark(&bpe, &text, iters);
        let speedup = single / parallel;

        let marker = if speedup > 1.1 && crossover.is_none() {
            crossover = Some(kb);
            " <-- crossover"
        } else {
            ""
        };

        println!(
            "{:>6}KB {:>8} {:>10.1}µs {:>10.1}µs {:>7.2}x{}",
            kb,
            tokens,
            single * 1_000_000.0,
            parallel * 1_000_000.0,
            speedup,
            marker
        );
    }

    println!();
    println!("Current threshold: 50KB");
    if let Some(kb) = crossover {
        println!("Measured crossover: ~{}KB", kb);
        if kb < 40 {
            println!(
                "Recommendation: threshold could be lowered to {}KB",
                kb.max(20)
            );
        } else if kb > 60 {
            println!("Recommendation: threshold could be raised to {}KB", kb);
        } else {
            println!("Recommendation: 50KB threshold is well-suited for this hardware");
        }
    }
}
