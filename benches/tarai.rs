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
    benchmark_group(
        (x, y, z),
        [
            "tarai_naive",
            "tarai_memo",
            "tarai_lazy_closure",
            "tarai_lazy_enum",
        ],
        expected,
        tarai::tarai_naive,
        tarai::tarai_memo,
        tarai::tarai_lazy_closure,
        tarai::tarai_lazy_enum,
    );
    benchmark_group(
        (x, y, z),
        [
            "tarai_naive_checked",
            "tarai_memo_checked",
            "tarai_lazy_closure_checked",
            "tarai_lazy_enum_checked",
        ],
        Some(expected),
        tarai::tarai_naive_checked,
        tarai::tarai_memo_checked,
        tarai::tarai_lazy_closure_checked,
        tarai::tarai_lazy_enum_checked,
    );
}

fn benchmark_underflow_case(x: i32, y: i32, z: i32, expected: Option<i32>) {
    println!("\nChecked input case ({x}, {y}, {z}):");
    benchmark_group(
        (x, y, z),
        [
            "tarai_naive_checked",
            "tarai_memo_checked",
            "tarai_lazy_closure_checked",
            "tarai_lazy_enum_checked",
        ],
        expected,
        tarai::tarai_naive_checked,
        tarai::tarai_memo_checked,
        tarai::tarai_lazy_closure_checked,
        tarai::tarai_lazy_enum_checked,
    );
}

fn benchmark_group<T, F1, F2, F3, F4>(
    input: (i32, i32, i32),
    names: [&str; 4],
    expected: T,
    implementation_1: F1,
    implementation_2: F2,
    implementation_3: F3,
    implementation_4: F4,
) where
    T: std::fmt::Debug + PartialEq + Copy,
    F1: Fn(i32, i32, i32) -> T + Copy,
    F2: Fn(i32, i32, i32) -> T + Copy,
    F3: Fn(i32, i32, i32) -> T + Copy,
    F4: Fn(i32, i32, i32) -> T + Copy,
{
    let (x, y, z) = input;
    for (name, result) in [
        (names[0], implementation_1(x, y, z)),
        (names[1], implementation_2(x, y, z)),
        (names[2], implementation_3(x, y, z)),
        (names[3], implementation_4(x, y, z)),
    ] {
        assert_eq!(
            result, expected,
            "{name}({x}, {y}, {z}) returned an unexpected result; refusing to benchmark"
        );
    }

    let iterations_1 = calibrate_iterations(implementation_1, x, y, z);
    let iterations_2 = calibrate_iterations(implementation_2, x, y, z);
    let iterations_3 = calibrate_iterations(implementation_3, x, y, z);
    let iterations_4 = calibrate_iterations(implementation_4, x, y, z);

    for round in 0..WARMUP_ROUNDS {
        if round % 2 == 0 {
            let _ = [
                run_batch(implementation_1, x, y, z, iterations_1),
                run_batch(implementation_2, x, y, z, iterations_2),
                run_batch(implementation_3, x, y, z, iterations_3),
                run_batch(implementation_4, x, y, z, iterations_4),
            ];
        } else {
            let _ = [
                run_batch(implementation_4, x, y, z, iterations_4),
                run_batch(implementation_3, x, y, z, iterations_3),
                run_batch(implementation_2, x, y, z, iterations_2),
                run_batch(implementation_1, x, y, z, iterations_1),
            ];
        }
    }

    let mut samples_1 = Vec::with_capacity(SAMPLE_ROUNDS);
    let mut samples_2 = Vec::with_capacity(SAMPLE_ROUNDS);
    let mut samples_3 = Vec::with_capacity(SAMPLE_ROUNDS);
    let mut samples_4 = Vec::with_capacity(SAMPLE_ROUNDS);
    for round in 0..SAMPLE_ROUNDS {
        if round % 2 == 0 {
            let elapsed = [
                run_batch(implementation_1, x, y, z, iterations_1),
                run_batch(implementation_2, x, y, z, iterations_2),
                run_batch(implementation_3, x, y, z, iterations_3),
                run_batch(implementation_4, x, y, z, iterations_4),
            ];
            samples_1.push(ns_per_call(elapsed[0], iterations_1));
            samples_2.push(ns_per_call(elapsed[1], iterations_2));
            samples_3.push(ns_per_call(elapsed[2], iterations_3));
            samples_4.push(ns_per_call(elapsed[3], iterations_4));
        } else {
            let elapsed = [
                run_batch(implementation_4, x, y, z, iterations_4),
                run_batch(implementation_3, x, y, z, iterations_3),
                run_batch(implementation_2, x, y, z, iterations_2),
                run_batch(implementation_1, x, y, z, iterations_1),
            ];
            samples_4.push(ns_per_call(elapsed[0], iterations_4));
            samples_3.push(ns_per_call(elapsed[1], iterations_3));
            samples_2.push(ns_per_call(elapsed[2], iterations_2));
            samples_1.push(ns_per_call(elapsed[3], iterations_1));
        }
    }

    report_benchmark(names[0], x, y, z, iterations_1, samples_1);
    report_benchmark(names[1], x, y, z, iterations_2, samples_2);
    report_benchmark(names[2], x, y, z, iterations_3, samples_3);
    report_benchmark(names[3], x, y, z, iterations_4, samples_4);
}

fn ns_per_call(elapsed: Duration, iterations: u64) -> f64 {
    elapsed.as_secs_f64() * 1_000_000_000.0 / iterations as f64
}

fn report_benchmark(name: &str, x: i32, y: i32, z: i32, iterations: u64, mut samples: Vec<f64>) {
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
