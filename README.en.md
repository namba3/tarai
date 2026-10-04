# Tak (Tarai) Function

[日本語](README.md) | [English](README.en.md)

## What is the Tarai function?

The Tarai function takes three integers, `x`, `y`, and `z`. It returns `y` when `x` is less than or equal to `y`; otherwise, it passes the results of three recursive calls as the arguments to another call.

```text
T(x, y, z) =
    y                                      (x <= y)
    T(T(x - 1, y, z), T(y - 1, z, x),
      T(z - 1, x, y))                      (x > y)
```

For example, `T(10, 5, 0)` returns `10`. Because recursive results become arguments to another call, naive recursion repeats many of the same calculations. This repository compares direct recursion, memoization, and lazy evaluation using closures and an enum.

See [TAK Function on Wolfram MathWorld](https://mathworld.wolfram.com/TAKFunction.html) for more detail.

This repository implements the Tak (Tarai) function in four ways:

- Naive recursion
- Memoized recursion
- Lazy evaluation using closures
- Lazy evaluation using an enum

Each implementation also has a checked variant: `tarai_naive_checked`, `tarai_memo_checked`, `tarai_lazy_closure_checked`, and `tarai_lazy_enum_checked`. They return `None` if a subtraction during evaluation would underflow `i32`; otherwise, they return `Some(result)`.
The lazy variants defer the third recursive call until it is needed. The closure
variant also defers the subtraction used to construct that call, while the enum
variant performs the subtraction first. As a result, the checked variants can
evaluate different subtractions at boundary inputs.

Large inputs can increase runtime and memory use, and deep recursion can exhaust
the stack. Memoization still uses recursion, so deep inputs can exhaust the stack.
Checked variants detect subtraction underflow; they do not limit runtime, memory
use, or stack use.

```rust
assert_eq!(tarai::tarai_memo_checked(10, 5, 0), Some(10));
assert_eq!(tarai::tarai_memo_checked(i32::MIN + 1, i32::MIN, 0), None);
```

## Usage

Call a library function to evaluate the Tak function:

```rust
fn main() {
    let input = (10, 5, 0);
    println!(
        "tarai_naive: {}",
        tarai::tarai_naive(input.0, input.1, input.2)
    );
    println!(
        "tarai_memo: {}",
        tarai::tarai_memo(input.0, input.1, input.2)
    );
    println!(
        "tarai_lazy_closure: {}",
        tarai::tarai_lazy_closure(input.0, input.1, input.2)
    );
    println!(
        "tarai_lazy_enum: {}",
        tarai::tarai_lazy_enum(input.0, input.1, input.2)
    );
}
```

## Tests

Tests run on stable Rust:

```sh
cargo test
```

## Benchmark

The benchmarks run on stable Rust. Each case is warmed up twice, then measured
seven times; the median and range are reported. Iterations are calibrated for
each input and implementation to target 100 ms per sample, capped at `1 << 24`
iterations. A sample can be shorter when it reaches that cap; the actual count
is printed with each result.
Implementations are called through static dispatch, without function pointers.
Normal cases scale `x` and `y` from `(6, 3, 0)` through `(14, 7, 0)`, and also
vary `y` and `z` while keeping `x = 10`. Every implementation uses the same
inputs, and results are grouped by input case.
Checked variants are also measured on a subtraction-underflow case.
Each sample alternates between forward and reverse implementation order, but measurements still run in one process. System load and CPU frequency changes can affect the results; treat the numbers as local references and avoid direct comparison across different environments.

```sh
cargo bench
```

### Results

The following results were measured with `cargo +stable bench` on 2026-10-05.
Values are the median of seven samples in ns/call. Iteration counts vary by
implementation and are calibrated to target about 100 ms per sample. Treat these
as reference values: CPU load and the state of the virtualized environment can
affect measurements.

- CPU: AMD Ryzen 9 9900X, 12 cores / 24 threads
- OS: Linux 6.18.40, WSL2
- Rust: stable `rustc 1.99.0 (b940084d7 2026-09-28)`
- Profile: optimized (`cargo bench`)

Unchecked implementations:

| Input `(x, y, z)` | Naive recursion | Memoized recursion | Closure lazy evaluation | Enum lazy evaluation |
|---|---:|---:|---:|---:|
| `(6, 3, 0)` | 668.39 | 2,632.56 | 41.16 | 31.77 |
| `(8, 4, 0)` | 12,024.06 | 4,560.17 | 78.92 | 60.27 |
| `(10, 5, 0)` | 337,105.09 | 7,635.25 | 130.44 | 96.29 |
| `(12, 6, 0)` | 11,939,468.56 | 12,310.66 | 159.66 | 132.60 |
| `(14, 7, 0)` | 605,238,023.00 | 16,916.51 | 244.37 | 183.27 |
| `(10, 7, 4)` | 637.07 | 2,802.00 | 46.28 | 34.49 |
| `(10, 5, 3)` | 1,390.85 | 3,031.60 | 43.41 | 34.94 |

Checked implementations on inputs that complete successfully:

| Input `(x, y, z)` | Naive recursion | Memoized recursion | Closure lazy evaluation | Enum lazy evaluation |
|---|---:|---:|---:|---:|
| `(6, 3, 0)` | 779.24 | 2,720.76 | 49.65 | 37.73 |
| `(8, 4, 0)` | 14,251.66 | 4,449.59 | 79.92 | 62.68 |
| `(10, 5, 0)` | 419,363.53 | 8,186.25 | 146.82 | 105.57 |
| `(12, 6, 0)` | 15,493,072.25 | 12,735.74 | 185.83 | 141.22 |
| `(14, 7, 0)` | 725,736,747.00 | 17,077.28 | 280.62 | 211.67 |
| `(10, 7, 4)` | 701.05 | 2,547.04 | 44.18 | 33.32 |
| `(10, 5, 3)` | 1,889.63 | 3,279.97 | 52.22 | 41.02 |

For the checked underflow input `(i32::MIN + 1, i32::MIN, 0)`, all
implementations return `None`. Their median times were:

| Naive recursion | Memoized recursion | Closure lazy evaluation | Enum lazy evaluation |
|---:|---:|---:|---:|
| 3.03 ns | 33.86 ns | 4.42 ns | 3.87 ns |
