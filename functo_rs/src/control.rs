//! Control functors are special case of [data functors][`crate::data`], which can take [`FnOnce`] as continuations.

use crate::data;
pub use crate::impls::*;

/// A type-level label to force qualified_do to use `control::Functor`-hierarchy.
pub struct AsControl<F>(std::marker::PhantomData<F>);

pub trait Functor: data::Functor {
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B;
}

impl<F: Functor> AsControl<F> {
    #[inline(always)]
    pub fn fmap<A, B, G>(f: G, fa: F::Container<A>) -> F::Container<B>
    where
        G: FnOnce(A) -> B,
    {
        <F as Functor>::fmap(f, fa)
    }
}

impl Functor for Identity {
    #[inline(always)]
    fn fmap<A, B, F>(f: F, a: A) -> B
    where
        F: FnOnce(A) -> B,
    {
        f(a)
    }
}

impl Functor for OptionFunctor {
    #[inline(always)]
    fn fmap<A, B, F>(f: F, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        fa.map(f)
    }
}

impl<E> Functor for ResultFunctor<E> {
    #[inline(always)]
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

impl<F: Pointed> AsControl<F> {
    #[inline(always)]
    pub fn pure<A>(a: A) -> F::Container<A> {
        <F as Pointed>::pure(a)
    }
}

impl Pointed for Identity {
    #[inline(always)]
    fn pure<A>(a: A) -> A {
        a
    }
}

impl Pointed for OptionFunctor {
    #[inline(always)]
    fn pure<A>(a: A) -> Option<A> {
        Some(a)
    }
}

impl<E> Pointed for ResultFunctor<E> {
    #[inline(always)]
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

    #[inline(always)]
    fn ap<A, B, F>(ff: Self::Container<F>, fa: Self::Container<A>) -> Self::Container<B>
    where
        F: FnOnce(A) -> B,
    {
        <Self as Apply>::zip_with(|f, a| f(a), ff, fa)
    }
}

impl<F: Apply> AsControl<F> {
    #[inline(always)]
    pub fn zip_with<A, B, C, G>(f: G, fa: F::Container<A>, fb: F::Container<B>) -> F::Container<C>
    where
        G: FnOnce(A, B) -> C,
    {
        <F as Apply>::zip_with(f, fa, fb)
    }

    #[inline(always)]
    pub fn ap<A, B, G>(ff: F::Container<G>, fa: F::Container<A>) -> F::Container<B>
    where
        G: FnOnce(A) -> B,
    {
        <F as Apply>::ap(ff, fa)
    }
}

impl Apply for Identity {
    #[inline(always)]
    fn zip_with<A, B, C, F>(f: F, a: A, b: B) -> C
    where
        F: FnOnce(A, B) -> C,
    {
        f(a, b)
    }
}

impl Apply for OptionFunctor {
    #[inline(always)]
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
    #[inline(always)]
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

    #[inline(always)]
    fn flatten<A>(ffa: Self::Container<Self::Container<A>>) -> Self::Container<A> {
        <Self as Monad>::and_then(ffa, |fa| fa)
    }
}

impl<F: Monad> AsControl<F> {
    #[inline(always)]
    pub fn and_then<A, B, G>(fa: F::Container<A>, f: G) -> F::Container<B>
    where
        G: FnOnce(A) -> F::Container<B>,
    {
        <F as Monad>::and_then(fa, f)
    }

    #[inline(always)]
    pub fn flatten<A>(ffa: F::Container<F::Container<A>>) -> F::Container<A> {
        <F as Monad>::flatten(ffa)
    }
}

impl Monad for Identity {
    #[inline(always)]
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        F: FnOnce(A) -> Self::Container<B>,
    {
        f(fa)
    }
}

impl Monad for OptionFunctor {
    #[inline(always)]
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        F: FnOnce(A) -> Self::Container<B>,
    {
        fa.and_then(f)
    }
}

impl<E> Monad for ResultFunctor<E> {
    #[inline(always)]
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        F: FnOnce(A) -> Self::Container<B>,
    {
        fa.and_then(f)
    }
}

pub trait MonadFail: Monad {
    fn fail<A>(msg: &str) -> Self::Container<A>;
}

impl<T: MonadFail> AsControl<T> {
    #[inline(always)]
    pub fn fail<A>(msg: &str) -> T::Container<A> {
        <T as MonadFail>::fail(msg)
    }
}

impl MonadFail for OptionFunctor {
    #[inline(always)]
    fn fail<A>(_msg: &str) -> Option<A> {
        None
    }
}

impl<E: From<String>> MonadFail for ResultFunctor<E> {
    #[inline(always)]
    fn fail<A>(msg: &str) -> Result<A, E> {
        Err(msg.to_string().into())
    }
}

pub trait Alternative: Apply + Pointed {
    fn empty<T>() -> Self::Container<T>;
    fn choice<T>(a: Self::Container<T>, b: Self::Container<T>) -> Self::Container<T>;

    #[inline(always)]
    fn guard(p: bool) -> Self::Container<()> {
        if p {
            <Self as Pointed>::pure(())
        } else {
            Self::empty()
        }
    }
}

impl<G: Alternative> AsControl<G> {
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
