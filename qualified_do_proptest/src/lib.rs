use proptest::prelude::*;
use proptest::strategy;
use std::fmt::Debug;

pub enum BoxedProptest {}

impl BoxedProptest {
    #[inline(always)]
    pub fn fmap<A, B, F>(f: F, fa: BoxedStrategy<A>) -> BoxedStrategy<B>
    where
        A: Debug + 'static,
        B: Debug,
        F: Fn(A) -> B + 'static,
    {
        fa.prop_map(f).boxed()
    }

    #[inline(always)]
    pub fn pure<A: Clone + Debug + 'static>(a: A) -> BoxedStrategy<A> {
        Just(a).boxed()
    }

    #[inline(always)]
    pub fn zip_with<A, B, C, F>(
        f: F,
        fa: BoxedStrategy<A>,
        fb: BoxedStrategy<B>,
    ) -> BoxedStrategy<C>
    where
        A: Debug + 'static,
        B: Debug + 'static,
        C: Debug,
        F: Fn(A, B) -> C + 'static,
    {
        (fa, fb).prop_map(move |(a, b)| f(a, b)).boxed()
    }

    #[inline(always)]
    pub fn and_then<A, B, F>(f: F, fa: BoxedStrategy<A>) -> BoxedStrategy<B>
    where
        A: Debug + 'static,
        B: Debug,
        F: Fn(A) -> BoxedStrategy<B> + 'static,
    {
        fa.prop_flat_map(f).boxed()
    }

    #[inline(always)]
    pub fn fail<T: Clone + Debug + 'static>(msg: &str) -> BoxedStrategy<T> {
        Just(Option::<T>::None)
            .prop_filter_map(msg.to_string(), |i| i)
            .boxed()
    }
}

type Mapped<S, T, A, B, C> = strategy::Map<(S, T), Box<dyn Fn((A, B)) -> C + 'static>>;

pub enum Proptest {}

impl Proptest {
    #[inline(always)]
    pub fn fmap<A, B, S, F>(f: F, fa: S) -> strategy::Map<S, F>
    where
        S: Strategy<Value = A>,
        A: Debug,
        B: Debug,
        F: Fn(A) -> B,
    {
        fa.prop_map(f)
    }

    #[inline(always)]
    pub fn pure<A: Clone + Debug + 'static>(a: A) -> strategy::Just<A> {
        Just(a)
    }

    #[inline(always)]
    pub fn zip_with<A, B, C, S, T, F>(f: F, fa: S, fb: T) -> Mapped<S, T, A, B, C>
    where
        A: Debug,
        B: Debug,
        C: Debug,
        S: Strategy<Value = A>,
        T: Strategy<Value = B>,
        F: Fn(A, B) -> C + 'static,
    {
        (fa, fb).prop_map(Box::new(move |(a, b)| f(a, b)))
    }

    #[inline(always)]
    pub fn and_then<A, B, S, T, F>(f: F, fa: S) -> strategy::Flatten<strategy::Map<S, F>>
    where
        A: Debug + 'static,
        B: Debug,
        S: Strategy<Value = A>,
        T: Strategy<Value = B>,
        F: Fn(A) -> T,
    {
        fa.prop_flat_map(f)
    }

    #[inline(always)]
    pub fn fail<T: Clone + Debug>(msg: &str) -> impl Strategy<Value = T> {
        Just(Option::<T>::None).prop_filter_map(msg.to_string(), |i| i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum Expr {
        Add(Box<Expr>, Box<Expr>),
        Mul(Box<Expr>, Box<Expr>),
        Num(i32),
    }

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

    proptest! {
        #[test]
        fn test_gen_expr(expr in gen_expr()) {
            println!("{:?}", expr);
        }
    }
}
