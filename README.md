# Experimenting around Functorial pattern in Rust

This repository aims at exploring the followings in Rust:

- Affine Data/Control Functor hierarchy
- Multiplicative Functor hierarchy
- A port of [`QualifiedDo`](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/qualified_do.html) in Haskell to Rust macro to use the above uniformly
  + Support [`ApplicativeDo`](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/applicative_do.html#extension-ApplicativeDo) and MonadFail as well.

## Showcase

```rust
use functo_rs::control::*;
use qualified_do::*;

let ans: Option<i64> = qdo! {Optioned {
    i <- Some(5);
    j <- Some(6);
    let k = 7i64;
    return i + j + k
}};
assert_eq!(ans, Some(18));
```

```rust
use functo_rs::control::*;
use qualified_do::*;

let ans: Option<i64> = qdo! {Optioned {
    i <- Some(5);
    j <- Some(6);
    _k <- None::<i64>;
    let k = 7i64;
    return i + j + k
}};
assert_eq!(ans, None);
```

```rust
use functo_rs::nonlinear::*;
use qualified_do_macro::qdo;
let is = vec![1, 2, 3];
let js = vec![4, 5, 6];

let ans: Vec<i64> = qdo! {UndetVec {
    i <- is.clone();
    j <- js.clone();
    let k = 100i64;
    UndetVec::guard(i % 2 == 1);
    return i + j + k
}};
assert_eq!(
    ans,
    is.into_iter()
        .flat_map(|i| js.iter().cloned().flat_map(move |j| if i % 2 == 1 {
            Some(i + j + 100)
        } else {
            None
        }))
        .collect::<Vec<_>>()
);
```

```rust
use functo_rs::data::*;
use qualified_do_macro::qdo;
let is = vec![1, 2, 3];
let js = vec![4, 5, 6];

let ans: Vec<i64> = qdo! {ZipVec {
    i <- is.clone();
    j <- js.clone();
    let k = 100i64;
    return i + j + k
}};
assert_eq!(
    ans,
    is.into_iter()
        .zip(js)
        .map(|(i, j)| i + j + 100)
        .collect::<Vec<_>>()
);
```

```rust
use either::Either::*;
let a = vec![Some(1), None, Some(3)];
let b = vec![Left(4), Left(5), Right(6)];
let answer = {
    let a = a.clone();
    let b = b.clone();
    qdo! { Iter {
        Some(x) <- a;
        Left(y) <- b.clone();
        let z = 100i64;
        return x + y + z
    }}
    .collect::<Vec<_>>()
};
let c = a
    .into_iter()
    .flatten()
    .flat_map(|x| {
        b.iter()
            .cloned()
            .flat_map(|x| x.left())
            .map(move |y| x + y + 100)
    })
    .collect::<Vec<_>>();
assert_eq!(answer, c);
```

```rust
fn gen_expr() -> impl Strategy<Value = Expr> {
    use qualified_do::qdo;
    let leaf = any::<i32>().prop_map(Expr::Num).boxed();
    leaf.prop_recursive(8, 256, 10, |inner| {
        prop_oneof![
            qdo! { BoxedProptest {
                l <- inner.clone();
                r <- inner.clone();
                return Expr::Add(l.into(), r.into())
            }},
            qdo! { BoxedProptest {
                l <- inner.clone();
                r <- inner.clone();
                return Expr::Mul(l.into(), r.into())
            }}
        ]
    })
}
```
