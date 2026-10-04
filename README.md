# 竹内関数（たらい回し関数）

[日本語](README.md) | [English](README.en.md)

竹内関数を次の 4 つの方法で実装しています。

- 素朴な再帰
- メモ化再帰
- クロージャーを使った遅延評価
- enum を使った遅延評価

## 使い方

ライブラリ関数を呼び出して竹内関数を計算できます。

```rust
fn main() {
    let result = tarai::tarai_naive(10, 5, 0);
    println!("{result}");
}
```

## テスト

stable Rust で実行できます。

```sh
cargo test
```

## ベンチマーク

stable Rust で実行できます。
各ケースを 2 回ウォームアップした後、7 回計測し、中央値と範囲を表示します。

```sh
cargo bench
```
