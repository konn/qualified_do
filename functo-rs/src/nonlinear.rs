pub use super::data::{Functor, Pointed};
use crate::impls::*;

pub trait Apply: Functor {
    fn zip_with<A, B, C, F, G>(
        f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        A: Clone,
        B: Clone,
        F: FnMut(A, B) -> C;
}

impl Apply for Identity {
    fn zip_with<A, B, C, F, G>(mut f: F, a: A, b: B) -> C
    where
        F: FnMut(A, B) -> C,
    {
        f(a, b)
    }
}

impl Apply for OptionFunctor {
    fn zip_with<A, B, C, F, G>(
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
    fn zip_with<A, B, C, F, G>(
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
    fn zip_with<A, B, C, F, G>(
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

impl Apply for UndetVec {
    fn zip_with<A, B, C, F, G>(
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
    fn zip_with<A, B, C, F, G>(
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

pub trait Monad: Apply + Pointed {
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        A: Clone,
        B: Clone,
        F: FnMut(A) -> Self::Container<B>;
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
