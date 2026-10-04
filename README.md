# 竹内関数（たらい回し関数）

[日本語](README.md) | [English](README.en.md)

竹内関数を次の 4 つの方法で実装しています。

- 素朴な再帰
- メモ化再帰
- クロージャーを使った遅延評価
- enum を使った遅延評価

## ベンチマーク

ベンチマークには nightly Rust が必要です。

```sh
cargo +nightly bench
```
