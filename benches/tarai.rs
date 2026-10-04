use std::{
    hint::black_box,
    time::{Duration, Instant},
};

const CASES: [((i32, i32, i32), i32); 7] = [
    // x と y を段階的に広げ、再帰量の増加を比較するケース。
    ((6, 3, 0), 6),
    ((8, 4, 0), 8),
    ((10, 5, 0), 10),
    ((12, 6, 0), 12),
    ((14, 7, 0), 14),
    // x を固定し、y と z の違いを比較するケース。
    ((10, 7, 4), 10),
    ((10, 5, 3), 10),
];
const UNDERFLOW_CASES: [((i32, i32, i32), Option<i32>); 1] = [((i32::MIN + 1, i32::MIN, 0), None)];

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
    println!(
        "Protocol: target sample {} ms, maximum {} iterations/sample, {} warmup rounds, {} measured samples",
        TARGET_SAMPLE_TIME.as_millis(),
        MAX_ITERATIONS,
        WARMUP_ROUNDS,
        SAMPLE_ROUNDS
    );
    println!("Benchmark results (nanoseconds per call):");

    for &((x, y, z), expected) in &CASES {
        benchmark_input_case(x, y, z, expected);
    }

    for &((x, y, z), expected) in &UNDERFLOW_CASES {
        benchmark_underflow_case(x, y, z, expected);
    }
}

fn benchmark_input_case(x: i32, y: i32, z: i32, expected: i32) {
    println!("\nInput case ({x}, {y}, {z}):");
    benchmark(x, y, z, "tarai_naive", tarai::tarai_naive, expected);
    benchmark(x, y, z, "tarai_memo", tarai::tarai_memo, expected);
    benchmark(
        x,
        y,
        z,
        "tarai_lazy_closure",
        tarai::tarai_lazy_closure,
        expected,
    );
    benchmark(x, y, z, "tarai_lazy_enum", tarai::tarai_lazy_enum, expected);
    benchmark(
        x,
        y,
        z,
        "tarai_naive_checked",
        tarai::tarai_naive_checked,
        Some(expected),
    );
    benchmark(
        x,
        y,
        z,
        "tarai_memo_checked",
        tarai::tarai_memo_checked,
        Some(expected),
    );
    benchmark(
        x,
        y,
        z,
        "tarai_lazy_closure_checked",
        tarai::tarai_lazy_closure_checked,
        Some(expected),
    );
    benchmark(
        x,
        y,
        z,
        "tarai_lazy_enum_checked",
        tarai::tarai_lazy_enum_checked,
        Some(expected),
    );
}

fn benchmark_underflow_case(x: i32, y: i32, z: i32, expected: Option<i32>) {
    println!("\nChecked input case ({x}, {y}, {z}):");
    benchmark(
        x,
        y,
        z,
        "tarai_naive_checked",
        tarai::tarai_naive_checked,
        expected,
    );
    benchmark(
        x,
        y,
        z,
        "tarai_memo_checked",
        tarai::tarai_memo_checked,
        expected,
    );
    benchmark(
        x,
        y,
        z,
        "tarai_lazy_closure_checked",
        tarai::tarai_lazy_closure_checked,
        expected,
    );
    benchmark(
        x,
        y,
        z,
        "tarai_lazy_enum_checked",
        tarai::tarai_lazy_enum_checked,
        expected,
    );
}

fn benchmark<T, F>(x: i32, y: i32, z: i32, name: &str, implementation: F, expected: T)
where
    T: std::fmt::Debug + PartialEq,
    F: Fn(i32, i32, i32) -> T + Copy,
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

fn calibrate_iterations<T, F>(implementation: F, x: i32, y: i32, z: i32) -> u64
where
    F: Fn(i32, i32, i32) -> T + Copy,
{
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

fn run_batch<T, F>(implementation: F, x: i32, y: i32, z: i32, iterations: u64) -> Duration
where
    F: Fn(i32, i32, i32) -> T + Copy,
{
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(implementation(black_box(x), black_box(y), black_box(z)));
    }
    start.elapsed()
}
