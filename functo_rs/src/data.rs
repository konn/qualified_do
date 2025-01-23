//! Data functors abstracts over data-like structures, which can cosume continuations as many times as they want.
//! Some of data functors can be [control functors][`crate::control`], which can consume continuations at most once.

use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub use crate::impls::*;

/// A type-level label to force qualified_do to use `data::Functor`-hierarchy.
pub struct AsData<F>(PhantomData<F>);

impl<G: Functor> AsData<G> {
    #[inline(always)]
    pub fn fmap<A, B, F>(f: F, fa: G::Container<A>) -> G::Container<B>
    where
        F: FnMut(A) -> B,
    {
        G::fmap(f, fa)
    }
}

pub trait Functor {
    type Container<T>;

    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B;
}

impl Functor for Identity {
    type Container<T> = T;

    #[inline(always)]
    fn fmap<A, B, F>(mut f: F, a: A) -> B
    where
        F: FnMut(A) -> B,
    {
        f(a)
    }
}

impl Functor for UndetVec {
    type Container<T> = Vec<T>;

    #[inline(always)]
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.into_iter().map(f).collect()
    }
}

impl Functor for ZipVec {
    type Container<T> = Vec<T>;

    #[inline(always)]
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.into_iter().map(f).collect()
    }
}

impl Functor for OptionFunctor {
    type Container<T> = Option<T>;

    #[inline(always)]
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.map(f)
    }
}

impl Functor for Boxed {
    type Container<T> = Box<T>;

    fn fmap<A, B, F>(mut f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        Box::new(f(*fa))
    }
}

impl<E> Functor for ResultFunctor<E> {
    type Container<T> = Result<T, E>;

    #[inline(always)]
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.map(f)
    }
}

impl Functor for V2 {
    type Container<T> = (T, T);

    #[inline(always)]
    fn fmap<A, B, F>(mut f: F, (a1, a2): Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        (f(a1), f(a2))
    }
}

impl<const N: usize> Functor for ArrayFunctor<N> {
    type Container<T> = [T; N];

    #[inline(always)]
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnMut(A) -> B,
    {
        fa.map(f)
    }
}

pub trait Pointed: Functor {
    fn pure<T: Clone>(t: T) -> Self::Container<T>;
}

impl<G: Pointed> AsData<G> {
    #[inline(always)]
    pub fn pure<T: Clone>(t: T) -> G::Container<T> {
        G::pure(t)
    }
}

impl Pointed for Identity {
    #[inline(always)]
    fn pure<T: Clone>(t: T) -> T {
        t
    }
}

impl Pointed for OptionFunctor {
    #[inline(always)]
    fn pure<T: Clone>(t: T) -> Option<T> {
        Some(t)
    }
}

impl<E> Pointed for ResultFunctor<E> {
    #[inline(always)]
    fn pure<T: Clone>(t: T) -> Result<T, E> {
        Ok(t)
    }
}

impl Pointed for UndetVec {
    #[inline(always)]
    fn pure<T: Clone>(t: T) -> Vec<T> {
        vec![t]
    }
}

impl Pointed for V2 {
    #[inline(always)]
    fn pure<T: Clone>(t: T) -> (T, T) {
        (t.clone(), t)
    }
}

impl Pointed for Boxed {
    #[inline(always)]
    fn pure<T: Clone>(t: T) -> Box<T> {
        Box::new(t)
    }
}

#[derive(Clone)]
pub(crate) struct WrapArrayStruct<U>(pub(crate) U);
impl<U> Debug for WrapArrayStruct<U> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<ELEMENT>")
    }
}

#[inline(always)]
pub(crate) fn unsafe_collect_array<const N: usize, I>(iter: I) -> [I::Item; N]
where
    I: Iterator,
{
    <[WrapArrayStruct<I::Item>; N]>::try_from(iter.map(WrapArrayStruct).collect::<Vec<_>>())
        .unwrap()
        .map(|WrapArrayStruct(t)| t)
}

impl<const N: usize> Pointed for ArrayFunctor<N> {
    fn pure<T: Clone>(t: T) -> [T; N] {
        unsafe_collect_array(itertools::repeat_n(t, N))
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

    #[inline(always)]
    fn ap<A, B, F>(ff: Self::Container<F>, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        Self::zip_with(|f, a| f(a), ff, fa)
    }
}

impl<G: Apply> AsData<G> {
    #[inline(always)]
    pub fn zip_with<A, B, C, F>(f: F, fa: G::Container<A>, fb: G::Container<B>) -> G::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        G::zip_with(f, fa, fb)
    }

    #[inline(always)]
    pub fn ap<A, B, F>(ff: G::Container<F>, fa: G::Container<A>) -> G::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        G::ap(ff, fa)
    }
}

impl Apply for Identity {
    #[inline(always)]
    fn zip_with<A, B, C, F>(mut f: F, a: A, b: B) -> C
    where
        F: FnMut(A, B) -> C,
    {
        f(a, b)
    }
}

impl Apply for OptionFunctor {
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn zip_with<A, B, C, F>(
        mut f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        fa.into_iter().zip(fb).map(|(a, b)| f(a, b)).collect()
    }
}

impl Apply for V2 {
    #[inline(always)]
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

impl Apply for Boxed {
    #[inline(always)]
    fn zip_with<A, B, C, F>(
        mut f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        Box::new(f(*fa, *fb))
    }
}

impl<const N: usize> Apply for ArrayFunctor<N> {
    #[inline(always)]
    fn zip_with<A, B, C, F>(
        mut f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnMut(A, B) -> C,
    {
        unsafe_collect_array(fa.into_iter().zip(fb).map(|(a, b)| f(a, b)))
    }
}

pub trait Alternative: Apply + Pointed {
    fn empty<T>() -> Self::Container<T>;
    fn choice<T>(a: Self::Container<T>, b: Self::Container<T>) -> Self::Container<T>;

    #[inline(always)]
    fn guard(p: bool) -> Self::Container<()> {
        if p {
            Self::pure(())
        } else {
            Self::empty()
        }
    }
}

impl<G: Alternative> AsData<G> {
    #[inline(always)]
    pub fn empty<T>() -> G::Container<T> {
        G::empty()
    }

    #[inline(always)]
    pub fn choice<T>(a: G::Container<T>, b: G::Container<T>) -> G::Container<T> {
        G::choice(a, b)
    }

    #[inline(always)]
    pub fn guard(p: bool) -> G::Container<()> {
        G::guard(p)
    }
}

impl Alternative for OptionFunctor {
    #[inline(always)]
    fn empty<T>() -> Option<T> {
        None
    }

    #[inline(always)]
    fn choice<T>(a: Self::Container<T>, b: Self::Container<T>) -> Self::Container<T> {
        a.or(b)
    }
}

impl<E: Default> Alternative for ResultFunctor<E> {
    #[inline(always)]
    fn empty<T>() -> Result<T, E> {
        Err(E::default())
    }

    #[inline(always)]
    fn choice<T>(a: Self::Container<T>, b: Self::Container<T>) -> Self::Container<T> {
        a.or(b)
    }
}
