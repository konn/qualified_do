//! Data functors abstracts over data-like structures, which can cosume continuations as many times as they want.
//! Some of data functors can be [control functors][`crate::control`], which can consume continuations at most once.

use std::marker::PhantomData;

use crate::impls::*;

/// A type-level label to force qualified_do to use `data::Functor`-hierarchy.
pub struct AsData<F>(PhantomData<F>);

impl<G: Functor> AsData<G> {
    pub fn fmap<A, B, F>(f: F, fa: G::Container<A>) -> G::Container<B>
    where
        F: Fn(A) -> B,
    {
        G::fmap(f, fa)
    }
}

pub trait Functor {
    type Container<T>;

    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: Fn(A) -> B;
}

impl Functor for Identity {
    type Container<T> = T;

    fn fmap<A, B, F>(mut f: F, a: A) -> B
    where
        F: FnMut(A) -> B,
    {
        f(a)
    }
}

impl Functor for UndetVec {
    type Container<T> = Vec<T>;

    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.into_iter().map(f).collect()
    }
}

impl Functor for ZipVec {
    type Container<T> = Vec<T>;

    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.into_iter().map(f).collect()
    }
}

impl Functor for OptionFunctor {
    type Container<T> = Option<T>;

    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.map(f)
    }
}

impl<E> Functor for ResultFunctor<E> {
    type Container<T> = Result<T, E>;

    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.map(f)
    }
}

impl Functor for V2 {
    type Container<T> = (T, T);

    fn fmap<A, B, F>(mut f: F, (a1, a2): Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        (f(a1), f(a2))
    }
}

pub trait Pointed: Functor {
    fn pure<T: Clone>(t: T) -> Self::Container<T>;
}

impl<G: Pointed> AsData<G> {
    pub fn pure<T: Clone>(t: T) -> G::Container<T> {
        G::pure(t)
    }
}

impl Pointed for Identity {
    fn pure<T: Clone>(t: T) -> T {
        t
    }
}

impl Pointed for OptionFunctor {
    fn pure<T: Clone>(t: T) -> Option<T> {
        Some(t)
    }
}

impl<E> Pointed for ResultFunctor<E> {
    fn pure<T: Clone>(t: T) -> Result<T, E> {
        Ok(t)
    }
}

impl Pointed for UndetVec {
    fn pure<T: Clone>(t: T) -> Vec<T> {
        vec![t]
    }
}

impl Pointed for V2 {
    fn pure<T: Clone>(t: T) -> (T, T) {
        (t.clone(), t)
    }
}

pub trait Apply: Functor {
    fn zip_with<A, B, C, F>(
        f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C;

    fn ap<A, B, F>(ff: Self::Container<F>, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        Self::zip_with(|f, a| f(a), ff, fa)
    }
}

impl<G: Apply> AsData<G> {
    pub fn zip_with<A, B, C, F>(f: F, fa: G::Container<A>, fb: G::Container<B>) -> G::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        G::zip_with(f, fa, fb)
    }

    pub fn ap<A, B, F>(ff: G::Container<F>, fa: G::Container<A>) -> G::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        G::ap(ff, fa)
    }
}

impl Apply for Identity {
    fn zip_with<A, B, C, F>(mut f: F, a: A, b: B) -> C
    where
        F: FnMut(A, B) -> C,
    {
        f(a, b)
    }
}

impl Apply for OptionFunctor {
    fn zip_with<A, B, C, F>(
        mut f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        fa.zip(fb).map(|(a, b)| f(a, b))
    }
}

impl<E> Apply for ResultFunctor<E> {
    fn zip_with<A, B, C, F>(
        mut f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        fa.and_then(|a| fb.map(|b| f(a, b)))
    }
}

impl Apply for ZipVec {
    fn zip_with<A, B, C, F>(
        mut f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        fa.into_iter()
            .zip(fb.into_iter())
            .map(|(a, b)| f(a, b))
            .collect()
    }
}

impl Apply for V2 {
    fn zip_with<A, B, C, F>(
        mut f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        (f(fa.0, fb.0), f(fa.1, fb.1))
    }
}

pub trait Alternative: Apply + Pointed {
    fn empty<T>() -> Self::Container<T>;
    fn choice<T>(a: Self::Container<T>, b: Self::Container<T>) -> Self::Container<T>;

    fn guard(p: bool) -> Self::Container<()> {
        if p {
            Self::pure(())
        } else {
            Self::empty()
        }
    }
}

impl<G: Alternative> AsData<G> {
    pub fn empty<T>() -> G::Container<T> {
        G::empty()
    }

    pub fn choice<T>(a: G::Container<T>, b: G::Container<T>) -> G::Container<T> {
        G::choice(a, b)
    }

    pub fn guard(p: bool) -> G::Container<()> {
        G::guard(p)
    }
}

impl Alternative for OptionFunctor {
    fn empty<T>() -> Option<T> {
        None
    }

    fn choice<T>(a: Self::Container<T>, b: Self::Container<T>) -> Self::Container<T> {
        a.or(b)
    }
}
