//! Control functors are special case of [data functors][`crate::data`], which can take [`FnOnce`] as continuations.

use crate::data;
use crate::impls::*;

pub trait Functor: data::Functor {
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B;
}

impl Functor for Identity {
    fn fmap<A, B, F>(f: F, a: A) -> B
    where
        F: FnOnce(A) -> B,
    {
        f(a)
    }
}

impl Functor for OptionFunctor {
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        fa.map(f)
    }
}

impl<E> Functor for ResultFunctor<E> {
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        fa.map(f)
    }
}

pub trait Pointed: Functor + data::Pointed {
    fn pure<A>(a: A) -> Self::Container<A>;
}

impl Pointed for Identity {
    fn pure<A>(a: A) -> A {
        a
    }
}

impl Pointed for OptionFunctor {
    fn pure<A>(a: A) -> Option<A> {
        Some(a)
    }
}

impl<E> Pointed for ResultFunctor<E> {
    fn pure<A>(a: A) -> Result<A, E> {
        Ok(a)
    }
}

pub trait Apply: Functor + data::Apply {
    fn zip_with<A, B, C, F>(
        f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnOnce(A, B) -> C;
}

impl Apply for Identity {
    fn zip_with<A, B, C, F>(f: F, a: A, b: B) -> C
    where
        F: FnOnce(A, B) -> C,
    {
        f(a, b)
    }
}

impl Apply for OptionFunctor {
    fn zip_with<A, B, C, F>(
        f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnOnce(A, B) -> C,
    {
        fa.zip(fb).map(|(a, b)| f(a, b))
    }
}

impl<E> Apply for ResultFunctor<E> {
    fn zip_with<A, B, C, F>(
        f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        F: FnOnce(A, B) -> C,
    {
        fa.and_then(|a| fb.map(|b| f(a, b)))
    }
}

pub trait Monad: Apply + Pointed {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        F: FnOnce(A) -> Self::Container<B>;
}

impl Monad for Identity {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        F: FnOnce(A) -> Self::Container<B>,
    {
        f(fa)
    }
}

impl Monad for OptionFunctor {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        F: FnOnce(A) -> Self::Container<B>,
    {
        fa.and_then(f)
    }
}

impl<E> Monad for ResultFunctor<E> {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        F: FnOnce(A) -> Self::Container<B>,
    {
        fa.and_then(f)
    }
}
