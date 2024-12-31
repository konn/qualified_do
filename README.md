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

## Syntax

The `qdo` macro has the following syntax:

```rust
qdo!{ NAMESPACE {
  stmt1;
  stmt2;
  ...
  last_stmt [;] // Last ; is optional and changes the return value
}}
```

- `NAMESPACE`: module or type path to _qualify_ control functions.
- `stmt`s are do-statement, which should be one of the followings:
  + `let pat = expr;`: let-statement for (non-effectful) local binding.
  + `return a`: which wraps (pure) value `a` into effectful context;

    * __NOTE__: This DOES NOT do any early return. It is interpreted as just a syntactic
                sugar around `NAMESPACE::pure(a)`.
  + `[~]pat <- expr`: _effectful_ local binding. Corresponding roughly to `NAMESPACE::and_then`
    * `~` is omittable; if `~` is specified, it tries to desugar into simple closure on infalliable pattern.
  + `guard expr`: guarding expression. Filters out `expr` is false. Desugared into `NAMESPACE::guard(expr)`.
  + `expr`: effectful expression, with its result discarded.
- `last_stmt` MUST either be `return expr` or `expr`.
  + If there is no `;` atfter `last_stmt`, the final effectful value(s) will be returned.
  + If `last_stmt` is followed by `;`, the values are discarded and replaced with `()` inside effectful context.

If `pat` is just a single identifier, it is desugared to a simple closure.
If the `pat` is falliable pattern, it desugars into closure with `match`-expression, with default value calls `NAMESPACE::fail` to report pattern-match failure.

Further more, if the following conditions are met, `qdo`-expression will be desugared in `ApplicativeDo`-mode, which desugars in terms of `NAMESPACE::fmap`, `NAMESPACE::zip_with`, and possibly `NAMESPACE::pure`:

1. All `stmt`s but `last_stmt` contains NO varibale bound in `qdo`-context,
2. All binding patterns are identifiers, not a compound pattern,
3. No `guard` condition in `stmtN` contains identifiers defined in `qdo`-context, and
4. The `last_stmt` is of form `return expr`, where `expr` can refer to any identifier in scope including those bound in qdo.

In `ApplicativeDo` mode, all binding can be chained independently so they are chained with `NAMESPACE::zip_with` and finally mapped with `fmap`[^1].

[^1]: In Haskell, `ApplicativeDo` uses `fmap`, `ap`, and `join`. The reason we don't use join is that `join` needs nested container, which has less availability in Rust than Haskell.

`ApplicativeDo` utilises the independence of each binding, so in some cases you need less `clone()`s.
