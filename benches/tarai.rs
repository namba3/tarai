use std::{
    hint::black_box,
    time::{Duration, Instant},
};

type TaraiFn = fn(i32, i32, i32) -> i32;
type CheckedTaraiFn = fn(i32, i32, i32) -> Option<i32>;

const CASES: [((i32, i32, i32), i32); 2] = [((10, 5, 0), 10), ((12, 6, 0), 12)];

const IMPLEMENTATIONS: [(&str, TaraiFn); 4] = [
    ("tarai_naive", tarai::tarai_naive),
    ("tarai_memo", tarai::tarai_memo),
    ("tarai_lazy_closure", tarai::tarai_lazy_closure),
    ("tarai_lazy_enum", tarai::tarai_lazy_enum),
];

const CHECKED_IMPLEMENTATIONS: [(&str, CheckedTaraiFn); 1] =
    [("tarai_memo_checked", tarai::tarai_memo_checked)];

const TARGET_SAMPLE_TIME: Duration = Duration::from_millis(100);
const MAX_ITERATIONS: u64 = 1 << 24;
const WARMUP_ROUNDS: usize = 2;
const SAMPLE_ROUNDS: usize = 7;

fn main() {
    println!("Rust: {}", env!("RUSTC_VERSION"));
    println!(
        "Target: {}-{}",
        std::env::consts::ARCH,
        std::env::consts::OS
    );
    println!(
        "Profile: {}",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "optimized"
        }
    );
    println!("Benchmark results (nanoseconds per call):");

    for &(name, implementation) in &IMPLEMENTATIONS {
        for &((x, y, z), expected) in &CASES {
            benchmark(x, y, z, name, implementation, expected);
        }
    }

    for &(name, implementation) in &CHECKED_IMPLEMENTATIONS {
        for &((x, y, z), expected) in &CASES {
            benchmark(x, y, z, name, implementation, Some(expected));
        }
    }
}

fn benchmark<T>(
    x: i32,
    y: i32,
    z: i32,
    name: &str,
    implementation: fn(i32, i32, i32) -> T,
    expected: T,
) where
    T: std::fmt::Debug + PartialEq,
{
    assert_eq!(
        implementation(x, y, z),
        expected,
        "{name}({x}, {y}, {z}) returned an unexpected result; refusing to benchmark"
    );

    let iterations = calibrate_iterations(implementation, x, y, z);

    for _ in 0..WARMUP_ROUNDS {
        run_batch(implementation, x, y, z, iterations);
    }

    let mut samples = (0..SAMPLE_ROUNDS)
        .map(|_| run_batch(implementation, x, y, z, iterations))
        .map(|elapsed| elapsed.as_secs_f64() * 1_000_000_000.0 / iterations as f64)
        .collect::<Vec<_>>();
    samples.sort_by(f64::total_cmp);

    let median = samples[samples.len() / 2];
    let minimum = samples[0];
    let maximum = samples[samples.len() - 1];
    let samples = samples
        .iter()
        .map(|sample| format!("{sample:.2}"))
        .collect::<Vec<_>>()
        .join(", ");
    println!(
        "{name}({x}, {y}, {z}): median {median:.2} ns/call, range {minimum:.2}–{maximum:.2}, {iterations} iterations/sample; samples=[{samples}]"
    );
}

fn calibrate_iterations<T>(implementation: fn(i32, i32, i32) -> T, x: i32, y: i32, z: i32) -> u64 {
    let mut iterations = 1;
    loop {
        if run_batch(implementation, x, y, z, iterations) >= TARGET_SAMPLE_TIME
            || iterations >= MAX_ITERATIONS
        {
            return iterations;
        }
        iterations = (iterations * 2).min(MAX_ITERATIONS);
    }
}

fn run_batch<T>(
    implementation: fn(i32, i32, i32) -> T,
    x: i32,
    y: i32,
    z: i32,
    iterations: u64,
) -> Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(implementation(black_box(x), black_box(y), black_box(z)));
    }
    start.elapsed()
}
