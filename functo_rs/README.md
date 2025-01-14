# functo_rs - Functor/Monad hiearchies in Rust

## TL;DR

If you want to use `do`-expressions in Rust, you can use [`qualified_do`][qdo] crate, which is partially built on top of (but not tightly coupled with) this crate. You can either provide custom `Applicative` / `Monad` instances or your own `zip_with`, `and_then` and so with more flexible type.

[qdo]: https://crates.io/crates/qualified_do

## What is this?

This crate provides three distinct hierarchies of Functors:

1. [Data Functors](https://docs.rs/functo_rs/0.0.0/functo_rs/data/index.html)
     + Container-like funcors and applicaives, which can contain multiple values but each consumed once.
     + There is no such things data `Monad`.
2. [Control Functors](https://docs.rs/functo_rs/0.0.0/functo_rs/control/index.html)
     + Control-structure-like functors, which contains _at-most one_ value.
     + Contrary to Linear Haskell, `Option` and `Result` can fall into this class!
3. [Nonlinear (or Unrestricted) Functors](https://docs.rs/functo_rs/0.0.0/functo_rs/nonlinear/index.html)
     + What is called (ordinary) Monad in Haskell.
     + They can contain as many values, and all elements can be used for multiple times.

For the distinction between data/control monad refer to the following article by Arnaud Spiwack (although they are discussing _linear_ functors, we have _affine_ functors instead):

> [A Tale of Two Functors or: How I Learned to Stop Worrying and Love Data and Control](https://www.tweag.io/blog/2020-01-16-data-vs-control/)
