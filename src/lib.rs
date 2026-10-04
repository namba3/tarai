/// 竹内関数を素朴な再帰で計算します。
///
/// 入力が大きい場合、同じ計算を繰り返すため非常に時間がかかることがあります。
/// すべての中間値が `i32` の範囲に収まる入力を指定してください。
///
/// # 使用例
///
/// ```
/// use tarai::{tarai_lazy_closure, tarai_lazy_enum, tarai_memo, tarai_naive};
///
/// let input = (10, 5, 0);
/// let expected = tarai_naive(input.0, input.1, input.2);
/// assert_eq!(expected, 10);
/// assert_eq!(tarai_memo(input.0, input.1, input.2), expected);
/// assert_eq!(tarai_lazy_closure(input.0, input.1, input.2), expected);
/// assert_eq!(tarai_lazy_enum(input.0, input.1, input.2), expected);
/// ```
pub fn tarai_naive(x: i32, y: i32, z: i32) -> i32 {
    if x <= y {
        y
    } else {
        tarai_naive(
            tarai_naive(x - 1, y, z),
            tarai_naive(y - 1, z, x),
            tarai_naive(z - 1, x, y),
        )
    }
}

/// 竹内関数をメモ化再帰で計算します。
///
/// メモは呼び出しごとに作成されます。大きな入力ではメモリ使用量が増えることがあります。
/// すべての中間値が `i32` の範囲に収まる入力を指定してください。
pub fn tarai_memo(x: i32, y: i32, z: i32) -> i32 {
    use std::collections::HashMap;
    let mut memo = HashMap::new();

    fn t(x: i32, y: i32, z: i32, memo: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
        if let Some(v) = memo.get(&(x, y, z)) {
            return *v;
        }

        let result = if x <= y {
            y
        } else {
            let a = t(x - 1, y, z, memo);
            let b = t(y - 1, z, x, memo);
            let c = t(z - 1, x, y, memo);
            t(a, b, c, memo)
        };

        memo.insert((x, y, z), result);
        result
    }

    t(x, y, z, &mut memo)
}

/// 竹内関数をメモ化再帰で計算し、中間値の減算アンダーフローを検出します。
///
/// `i32` の減算が範囲外になる場合は `None` を返します。メモは呼び出しごとに作成されます。
pub fn tarai_memo_checked(x: i32, y: i32, z: i32) -> Option<i32> {
    use std::collections::HashMap;

    fn t(x: i32, y: i32, z: i32, memo: &mut HashMap<(i32, i32, i32), i32>) -> Option<i32> {
        let key = (x, y, z);
        if let Some(&value) = memo.get(&key) {
            return Some(value);
        }

        let result = if x <= y {
            y
        } else {
            let a = t(x.checked_sub(1)?, y, z, memo)?;
            let b = t(y.checked_sub(1)?, z, x, memo)?;
            let c = t(z.checked_sub(1)?, x, y, memo)?;
            t(a, b, c, memo)?
        };

        memo.insert(key, result);
        Some(result)
    }

    t(x, y, z, &mut HashMap::new())
}

/// 第 3 引数をクロージャーで遅延評価しながら竹内関数を計算します。
///
/// すべての中間値が `i32` の範囲に収まる入力を指定してください。
pub fn tarai_lazy_closure(x: i32, y: i32, z: i32) -> i32 {
    fn t(x: i32, y: i32, z: &dyn Fn() -> i32) -> i32 {
        if x <= y {
            y
        } else {
            let z = z();
            let a = t(x - 1, y, &|| z);
            let b = t(y - 1, z, &|| x);
            let c = || t(z - 1, x, &|| y);
            t(a, b, &c)
        }
    }

    t(x, y, &|| z)
}

/// 第 3 引数を enum で遅延評価しながら竹内関数を計算します。
///
/// すべての中間値が `i32` の範囲に収まる入力を指定してください。
pub fn tarai_lazy_enum(x: i32, y: i32, z: i32) -> i32 {
    enum V {
        Args { x: i32, y: i32, z: i32 },
        Result(i32),
    }
    impl V {
        pub fn eval(self) -> i32 {
            match self {
                V::Args { x, y, z } => t(x, y, V::Result(z)),
                V::Result(v) => v,
            }
        }
    }

    fn t(x: i32, y: i32, z: V) -> i32 {
        if x <= y {
            y
        } else {
            let z = z.eval();
            let a = t(x - 1, y, V::Result(z));
            let b = t(y - 1, z, V::Result(x));
            let c = V::Args {
                x: z - 1,
                y: x,
                z: y,
            };
            t(a, b, c)
        }
    }

    t(x, y, V::Result(z))
}

#[cfg(test)]
mod tests {
    const CASES: [((i32, i32, i32), i32); 8] = [
        ((0, 0, 0), 0),
        ((1, 2, 3), 2),
        ((-1, 0, -2), 0),
        ((2, 1, 0), 2),
        ((3, 2, 1), 3),
        ((3, 1, 2), 2),
        ((10, 5, 0), 10),
        ((12, 6, 0), 12),
    ];

    macro_rules! test {
        ($fn:ident) => {
            mod $fn {
                #[test]
                fn matches_expected_cases() {
                    for &((x, y, z), expected) in super::CASES.iter() {
                        let actual = super::super::$fn(x, y, z);
                        assert_eq!(actual, expected, "{}({}, {}, {})", stringify!($fn), x, y, z);
                    }
                }
            }
        };
    }

    test!(tarai_naive);
    test!(tarai_memo);
    test!(tarai_lazy_closure);
    test!(tarai_lazy_enum);

    #[test]
    fn checked_memo_matches_expected_cases() {
        for &((x, y, z), expected) in CASES.iter() {
            assert_eq!(super::tarai_memo_checked(x, y, z), Some(expected));
        }
    }

    #[test]
    fn checked_memo_reports_subtraction_underflow() {
        assert_eq!(super::tarai_memo_checked(i32::MIN + 1, i32::MIN, 0), None);
    }

    #[test]
    fn checked_memo_accepts_extreme_base_case() {
        assert_eq!(
            super::tarai_memo_checked(i32::MIN, i32::MIN, i32::MIN),
            Some(i32::MIN)
        );
    }

    #[test]
    fn implementations_match_on_small_input_domain() {
        for x in -1..=2 {
            for y in -1..=2 {
                for z in -1..=2 {
                    let expected = super::tarai_naive(x, y, z);
                    assert_eq!(super::tarai_memo(x, y, z), expected, "memo({x}, {y}, {z})");
                    assert_eq!(
                        super::tarai_lazy_closure(x, y, z),
                        expected,
                        "lazy_closure({x}, {y}, {z})"
                    );
                    assert_eq!(
                        super::tarai_lazy_enum(x, y, z),
                        expected,
                        "lazy_enum({x}, {y}, {z})"
                    );
                }
            }
        }
    }
}
