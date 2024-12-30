//! This crate provides abstractions over data and control functors,
//! as described in [A Tale of Two Functors or: How I Learned to Stop Worrying and Love Data and Ccontrol](https://www.tweag.io/blog/2020-01-16-data-vs-control/).
//! The original article takes _linear_ types into account, but in Rust we have to do with _affine_ types.
//! This difference means `Option` and `Result` _CAN_ be control functors, which are not in linear case.

pub mod impls;

pub mod control;

pub mod data;

pub mod nonlinear;
