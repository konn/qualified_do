pub use super::data::{Functor, Pointed};
use crate::data::unsafe_collect_array;
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
    #[inline(always)]
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
    #[inline(always)]
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

impl Apply for UndetVec {
    #[inline(always)]
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
    #[inline(always)]
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

impl<const N: usize> Apply for ArrayFunctor<N> {
    #[inline(always)]
    fn zip_with<A, B, C, F>(
        f: F,
        fa: Self::Container<A>,
        fb: Self::Container<B>,
    ) -> Self::Container<C>
    where
        A: Clone,
        B: Clone,
        F: FnMut(A, B) -> C,
    {
        <Self as crate::data::Apply>::zip_with(f, fa, fb)
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

impl<F: Alternative> AsNonlinear<F> {
    #[inline(always)]
    pub fn empty<T>() -> F::Container<T> {
        <F as Alternative>::empty()
    }

    #[inline(always)]
    pub fn choice<T>(a: F::Container<T>, b: F::Container<T>) -> F::Container<T> {
        <F as Alternative>::choice(a, b)
    }

    #[inline(always)]
    pub fn guard(p: bool) -> F::Container<()> {
        <F as Alternative>::guard(p)
    }
}

impl Alternative for OptionFunctor {
    #[inline(always)]
    fn empty<T>() -> Self::Container<T> {
        None
    }

    #[inline(always)]
    fn choice<T>(a: Self::Container<T>, b: Self::Container<T>) -> Self::Container<T> {
        a.or(b)
    }
}

impl Alternative for UndetVec {
    #[inline(always)]
    fn empty<T>() -> Self::Container<T> {
        vec![]
    }

    #[inline(always)]
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

    #[inline(always)]
    fn flatten<A>(ffa: Self::Container<Self::Container<A>>) -> Self::Container<A>
    where
        A: Clone,
        Self::Container<A>: Clone,
    {
        <Self as Monad>::and_then(ffa, |fa| fa)
    }
}

impl<F: Monad> AsNonlinear<F> {
    #[inline(always)]
    pub fn and_then<A: Clone, B: Clone, G>(fa: F::Container<A>, f: G) -> F::Container<B>
    where
        G: FnMut(A) -> F::Container<B>,
    {
        <F as Monad>::and_then(fa, f)
    }

    #[inline(always)]
    pub fn flatten<A: Clone>(ffa: F::Container<F::Container<A>>) -> F::Container<A>
    where
        F::Container<A>: Clone,
    {
        <F as Monad>::flatten(ffa)
    }
}

impl Monad for Identity {
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn and_then<A, B, F>(fa: Self::Container<A>, f: F) -> Self::Container<B>
    where
        A: Clone,
        B: Clone,
        F: FnMut(A) -> Self::Container<B>,
    {
        fa.into_iter().flat_map(f).collect()
    }
}

/// Takes diagonal
impl Monad for V2 {
    #[inline(always)]
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

/// Takes diagonal upon joining
impl<const N: usize> Monad for ArrayFunctor<N> {
    #[inline(always)]
    fn and_then<A, B, F>(xs: Self::Container<A>, mut f: F) -> Self::Container<B>
    where
        A: Clone,
        B: Clone,
        F: FnMut(A) -> Self::Container<B>,
    {
        unsafe_collect_array(
            xs.into_iter()
                .enumerate()
                .flat_map(|(i, x)| f(x).into_iter().nth(i)),
        )
    }
}

pub trait MonadFail: Monad {
    fn fail<A>(msg: &str) -> Self::Container<A>;
}

impl<T: MonadFail> AsNonlinear<T> {
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

impl MonadFail for UndetVec {
    #[inline(always)]
    fn fail<A>(_msg: &str) -> Vec<A> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_array_monad() {
        assert_eq!(
            ArrayFunctor::and_then([1, 2, 3, 4, 5], |x| [x, x + 1, x + 2, x + 3, x + 4]).to_vec(),
            [1, 2, 3, 4, 5]
                .into_iter()
                .enumerate()
                .map(|(i, x)| i + x)
                .collect::<Vec<_>>()
        )
    }

    #[test]
    fn test_array_zip_with() {
        assert_eq!(
            ArrayFunctor::zip_with(|a, b| a + b, [1, 2, 3], [4, 5, 6]).to_vec(),
            ArrayFunctor::and_then([1, 2, 3], |a| ArrayFunctor::fmap(|b| a + b, [4, 5, 6]))
        )
    }
}
