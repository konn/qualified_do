pub use super::data::{Functor, Pointed};
pub use crate::impls::*;

pub struct AsNonlinear<F>(std::marker::PhantomData<F>);

impl<F: Functor> AsNonlinear<F> {
    pub fn fmap<A, B, G>(f: G, fa: F::Container<A>) -> F::Container<B>
    where
        G: Fn(A) -> B,
    {
        <F as Functor>::fmap(f, fa)
    }
}

impl<F: Pointed> AsNonlinear<F> {
    pub fn pure<T: Clone>(t: T) -> F::Container<T> {
        <F as Pointed>::pure(t)
    }
}

pub trait Apply: Functor {
    fn zip_with<A, B, C, F>(
        f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        A: Clone,
        B: Clone,
        F: FnMut(A, B) -> C;
}

impl<F: Apply> AsNonlinear<F> {
    pub fn zip_with<A, B, C, G>(f: G, fa: F::Container<A>, fb: F::Container<B>) -> F::Container<C>
    where
        A: Clone,
        B: Clone,
        G: FnMut(A, B) -> C,
    {
        <F as Apply>::zip_with(f, fa, fb)
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
        fa.into_iter().zip(fb).map(|(a, b)| f(a, b)).collect()
    }
}

impl Apply for UndetVec {
    fn zip_with<A, B, C, F>(
        mut f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        A: Clone,
        B: Clone,
        F: FnMut(A, B) -> C,
    {
        fa.into_iter()
            .flat_map(|a| fb.iter().map(move |b| (a.clone(), b.clone())))
            .map(|(a, b)| f(a, b))
            .collect()
    }
}

impl Apply for V2 {
    fn zip_with<A, B, C, F>(
        mut f: F,
        (a, b): Self::Container<A>,
        (c, d): Self::Container<B>,
    ) -> Self::Container<C>
    where
        A: Clone,
        B: Clone,
        F: FnMut(A, B) -> C,
    {
        (f(a, c), f(b, d))
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

impl<F: Alternative> AsNonlinear<F> {
    pub fn empty<T>() -> F::Container<T> {
        <F as Alternative>::empty()
    }

    pub fn choice<T>(a: F::Container<T>, b: F::Container<T>) -> F::Container<T> {
        <F as Alternative>::choice(a, b)
    }

    pub fn guard(p: bool) -> F::Container<()> {
        <F as Alternative>::guard(p)
    }
}

impl Alternative for OptionFunctor {
    fn empty<T>() -> Self::Container<T> {
        None
    }

    fn choice<T>(a: Self::Container<T>, b: Self::Container<T>) -> Self::Container<T> {
        a.or(b)
    }
}

impl Alternative for UndetVec {
    fn empty<T>() -> Self::Container<T> {
        vec![]
    }

    fn choice<T>(mut a: Self::Container<T>, b: Self::Container<T>) -> Self::Container<T> {
        a.extend(b);
        a
    }
}

pub trait Monad: Apply + Pointed {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        A: Clone,
        B: Clone,
        F: FnMut(A) -> Self::Container<B>;

    fn flatten<A>(ffa: Self::Container<Self::Container<A>>) -> Self::Container<A>
    where
        A: Clone,
        Self::Container<A>: Clone,
    {
        <Self as Monad>::and_then(ffa, |fa| fa)
    }
}

impl<F: Monad> AsNonlinear<F> {
    pub fn and_then<A: Clone, B: Clone, G>(fa: F::Container<A>, f: G) -> F::Container<B>
    where
        G: FnMut(A) -> F::Container<B>,
    {
        <F as Monad>::and_then(fa, f)
    }

    pub fn flatten<A: Clone>(ffa: F::Container<F::Container<A>>) -> F::Container<A>
    where
        F::Container<A>: Clone,
    {
        <F as Monad>::flatten(ffa)
    }
}

impl Monad for Identity {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        A: Clone,
        B: Clone,
        F: FnOnce(A) -> Self::Container<B>,
    {
        f(fa)
    }
}

impl Monad for OptionFunctor {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        A: Clone,
        B: Clone,
        F: FnOnce(A) -> Self::Container<B>,
    {
        fa.and_then(f)
    }
}

impl<E> Monad for ResultFunctor<E> {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        A: Clone,
        B: Clone,
        F: FnOnce(A) -> Self::Container<B>,
    {
        fa.and_then(f)
    }
}

impl Monad for UndetVec {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        A: Clone,
        B: Clone,
        F: FnMut(A) -> Self::Container<B>,
    {
        fa.into_iter().flat_map(f).collect()
    }
}

impl Monad for V2 {
    fn and_then<A, B, F>((a, b): Self::Container<A>, mut f: F) -> Self::Container<B>
    where
        A: Clone,
        B: Clone,
        F: FnMut(A) -> Self::Container<B>,
    {
        let (a, _) = f(a);
        let (_, b) = f(b);
        (a, b)
    }
}
