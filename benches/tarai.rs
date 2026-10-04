use std::{
    hint::black_box,
    time::{Duration, Instant},
};

type TaraiFn = fn(i32, i32, i32) -> i32;

const CASES: [((i32, i32, i32), &str, TaraiFn); 8] = [
    ((10, 5, 0), "tarai_naive", tarai::tarai_naive),
    ((12, 6, 0), "tarai_naive", tarai::tarai_naive),
    ((10, 5, 0), "tarai_memo", tarai::tarai_memo),
    ((12, 6, 0), "tarai_memo", tarai::tarai_memo),
    ((10, 5, 0), "tarai_lazy_closure", tarai::tarai_lazy_closure),
    ((12, 6, 0), "tarai_lazy_closure", tarai::tarai_lazy_closure),
    ((10, 5, 0), "tarai_lazy_enum", tarai::tarai_lazy_enum),
    ((12, 6, 0), "tarai_lazy_enum", tarai::tarai_lazy_enum),
];

const TARGET_SAMPLE_TIME: Duration = Duration::from_millis(250);
const MAX_ITERATIONS: u64 = 1 << 24;

fn main() {
    println!("Benchmark results (approximate nanoseconds per call):");

    for &((x, y, z), name, implementation) in &CASES {
        let mut iterations = 1;

        loop {
            let start = Instant::now();
            for _ in 0..iterations {
                black_box(implementation(black_box(x), black_box(y), black_box(z)));
            }
            let elapsed = start.elapsed();

            if elapsed >= TARGET_SAMPLE_TIME || iterations >= MAX_ITERATIONS {
                let ns_per_call = elapsed.as_nanos() as f64 / iterations as f64;
                println!(
                    "{name}({x}, {y}, {z}): {ns_per_call:.2} ns/call ({iterations} iterations)"
                );
                break;
            }

            iterations = (iterations * 2).min(MAX_ITERATIONS);
        }
    }
}
