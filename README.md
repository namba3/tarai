# 竹内関数（たらい回し関数）

[日本語](README.md) | [English](README.en.md)

## 竹内関数とは

竹内関数（Tarai 関数）は、3 つの整数 `x`, `y`, `z` を受け取る再帰関数です。`x` が `y` 以下なら `y` を返し、そうでなければ 3 つの再帰呼び出しの結果を次の呼び出しに渡します。

```text
T(x, y, z) =
    y                                      (x <= y)
    T(T(x - 1, y, z), T(y - 1, z, x),
      T(z - 1, x, y))                      (x > y)
```

たとえば `T(10, 5, 0)` は `10` を返します。再帰呼び出しの結果が次の引数になるため、素朴な再帰では同じ計算が何度も行われます。このリポジトリでは、素朴な再帰、メモ化、クロージャーと enum による遅延評価の違いを比較します。

詳細は [Wolfram MathWorld の TAK Function](https://mathworld.wolfram.com/TAKFunction.html) を参照してください。

竹内関数を次の 4 つの方法で実装しています。

- 素朴な再帰
- メモ化再帰
- クロージャーを使った遅延評価
- enum を使った遅延評価

各方式には、減算のアンダーフローを検出する checked 版もあります。`tarai_naive_checked`、`tarai_memo_checked`、`tarai_lazy_closure_checked`、`tarai_lazy_enum_checked` は、計算途中で `i32` の範囲を超える減算が必要になった場合に `None` を返し、それ以外は `Some(結果)` を返します。

```rust
assert_eq!(tarai::tarai_memo_checked(10, 5, 0), Some(10));
assert_eq!(tarai::tarai_memo_checked(i32::MIN + 1, i32::MIN, 0), None);
```

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
関数ポインターを介さず、各実装を静的ディスパッチで呼び出します。
通常ケースには、`(6, 3, 0)` から `(14, 7, 0)` まで `x` と `y` を段階的に広げる入力と、`x = 10` を固定して `y` と `z` を変える入力を使います。全実装で同じ入力を計測します。
checked 版では、正常に計算できるケースに加えて減算アンダーフローのケースも計測します。

```sh
cargo bench
```
