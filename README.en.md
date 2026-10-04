# Tak (Tarai) Function

[日本語](README.md) | [English](README.en.md)

This repository implements the Tak (Tarai) function in four ways:

- Naive recursion
- Memoized recursion
- Lazy evaluation using closures
- Lazy evaluation using an enum

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

```sh
cargo bench
```
