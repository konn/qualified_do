use proptest::prelude::*;
use proptest::strategy;
use std::fmt::Debug;

pub enum BoxedProptest {}

impl BoxedProptest {
    pub fn fmap<A, B, F>(f: F, fa: BoxedStrategy<A>) -> BoxedStrategy<B>
    where
        A: Debug + 'static,
        B: Debug,
        F: Fn(A) -> B + 'static,
    {
        fa.prop_map(f).boxed()
    }

    pub fn pure<A: Clone + Debug + 'static>(a: A) -> BoxedStrategy<A> {
        Just(a).boxed()
    }

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

    pub fn and_then<A, B, F>(f: F, fa: BoxedStrategy<A>) -> BoxedStrategy<B>
    where
        A: Debug + 'static,
        B: Debug,
        F: Fn(A) -> BoxedStrategy<B> + 'static,
    {
        fa.prop_flat_map(f).boxed()
    }
}

type Mapped<S, T, A, B, C> = strategy::Map<(S, T), Box<dyn Fn((A, B)) -> C + 'static>>;

pub enum Proptest {}

impl Proptest {
    pub fn fmap<A, B, S, F>(f: F, fa: S) -> strategy::Map<S, F>
    where
        S: Strategy<Value = A> + 'static,
        A: Debug + 'static,
        B: Debug,
        F: Fn(A) -> B + 'static,
    {
        fa.prop_map(f)
    }

    pub fn pure<A: Clone + Debug + 'static>(a: A) -> strategy::Just<A> {
        Just(a)
    }

    pub fn zip_with<A, B, C, S, T, F>(f: F, fa: S, fb: T) -> Mapped<S, T, A, B, C>
    where
        A: Debug + 'static,
        B: Debug + 'static,
        C: Debug,
        S: Strategy<Value = A> + 'static,
        T: Strategy<Value = B> + 'static,
        F: Fn(A, B) -> C + 'static,
    {
        (fa, fb).prop_map(Box::new(move |(a, b)| f(a, b)))
    }

    pub fn and_then<A, B, S, T, F>(f: F, fa: S) -> strategy::Flatten<strategy::Map<S, F>>
    where
        A: Debug + 'static,
        B: Debug,
        S: Strategy<Value = A> + 'static,
        T: Strategy<Value = B> + 'static,
        F: Fn(A) -> T + 'static,
    {
        fa.prop_flat_map(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    enum Expr {
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
