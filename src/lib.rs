#![feature(test)]

/// 実直な再帰での実装
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

/// メモ化再帰での実装
pub fn tarai_memo(x: i32, y: i32, z: i32) -> i32 {
    use std::collections::HashMap;
    let mut memo = HashMap::new();

    fn t(x: i32, y: i32, z: i32, memo: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
        if let Some(v) = memo.get(&(x, y, z)) {
            return *v;
        }

        if x <= y {
            y
        } else {
            macro_rules! get_or_call {
                ($x:expr,$y:expr,$z:expr) => {
                    if let Some(v) = memo.get(&($x, $y, $z)) {
                        *v
                    } else {
                        let v = t($x, $y, $z, memo);
                        memo.insert(($x, $y, $z), v);
                        v
                    }
                };
            }

            let a = get_or_call!(x - 1, y, z);
            let b = get_or_call!(y - 1, z, x);
            let c = get_or_call!(z - 1, x, y);
            get_or_call!(a, b, c)
        }
    }

    t(x, y, z, &mut memo)
}

/// 遅延評価での実装 (クロージャー使用)
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

/// 遅延評価での実装 (enum 使用)
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
    macro_rules! case {
        (a $case:ident, $fn:ident, $outs:expr, ($($ins:expr),*)) => {
            #[test]
            fn $case() {
                let expected = $outs;
                let actual = super::super::$fn($($ins),*);
                assert_eq!(actual, expected);
            }
        };
        ($case:ident, $fn:ident, $ins:tt, $outs:expr) => {
            case!(a $case, $fn, $outs, $ins);
        }
    }
    macro_rules! test {
        ($fn:ident) => {
            mod $fn {
                case!(case_1, $fn, (10, 5, 0), 10);
                case!(case_2, $fn, (12, 6, 0), 12);
            }
        };
    }

    test!(tarai_naive);
    test!(tarai_memo);
    test!(tarai_lazy_closure);
    test!(tarai_lazy_enum);
}

#[cfg(test)]
mod benchs {
    macro_rules! case {
        ($case:ident, $fn:ident, ($($ins:expr),*)) => {
            #[bench]
            fn $case(b: &mut test::Bencher) {
                let (x,y,z) = ($($ins),*);
                b.iter(|| {
                    super::super::$fn(x, y, z)
                })
            }
        };
    }
    macro_rules! bench {
        ($fn:ident) => {
            mod $fn {
                extern crate test;
                case!(case_10_5_0, $fn, (10, 5, 0));
                case!(case_12_6_0, $fn, (12, 6, 0));
            }
        };
    }

    bench!(tarai_naive);
    bench!(tarai_memo);
    bench!(tarai_lazy_closure);
    bench!(tarai_lazy_enum);
}
