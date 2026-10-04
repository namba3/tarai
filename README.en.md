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

```rust
assert_eq!(tarai::tarai_memo_checked(10, 5, 0), Some(10));
assert_eq!(tarai::tarai_memo_checked(i32::MIN + 1, i32::MIN, 0), None);
```

## Usage

Call a library function to evaluate the Tak function:

```rust
fn main() {
    let result = tarai::tarai_naive(10, 5, 0);
    println!("{result}");
}
```

## Tests

Tests run on stable Rust:

```sh
cargo test
```

## Benchmark

The benchmarks run on stable Rust. Each case is warmed up twice, then measured
seven times; the median and range are reported.
Implementations are called through static dispatch, without function pointers.

```sh
cargo bench
```
