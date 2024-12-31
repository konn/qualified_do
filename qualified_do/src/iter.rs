use core::str;
use itertools::Itertools;
use std::iter::Iterator;
use std::iter::*;
pub enum ZipIter {}

impl ZipIter {
    pub fn fmap<'a, A, O, F>(f: F, a: A) -> Box<dyn Iterator<Item = O> + 'a>
    where
        A: IntoIterator + 'a,
        F: FnMut(A::Item) -> O + 'a,
    {
        Box::new(a.into_iter().map(f))
    }

    pub fn pure<'a, A: Clone + 'a>(a: A) -> Box<dyn Iterator<Item = A> + 'a> {
        Box::new(repeat(a))
    }

    pub fn zip_with<'a, A, B, C, F>(mut f: F, a: A, b: B) -> Box<dyn Iterator<Item = C> + 'a>
    where
        A: IntoIterator + 'a,
        B: IntoIterator + 'a,
        F: FnMut(A::Item, B::Item) -> C + 'a,
    {
        Box::new(a.into_iter().zip(b).map(move |(a, b)| f(a, b)))
    }

    pub fn empty<'a, A: 'a>() -> Box<dyn Iterator<Item = A> + 'a> {
        Box::new(empty())
    }

    pub fn guard(cond: bool) -> Box<dyn Iterator<Item = ()>> {
        if cond {
            Box::new(repeat(()))
        } else {
            Box::new(empty())
        }
    }

    pub fn choice<'a, L, R>(l: L, r: R) -> Box<dyn Iterator<Item = L::Item> + 'a>
    where
        L: IntoIterator + 'a,
        R: IntoIterator<Item = L::Item> + 'a,
    {
        use itertools::EitherOrBoth;
        Box::new(l.into_iter().zip_longest(r).map(|e| match e {
            EitherOrBoth::Left(e) | EitherOrBoth::Both(e, _) => e,
            EitherOrBoth::Right(e) => e,
        }))
    }
}
pub enum Iter {}

impl Iter {
    pub fn fmap<'a, A, O, F>(f: F, a: A) -> Box<dyn Iterator<Item = O> + 'a>
    where
        A: IntoIterator + 'a,
        F: FnMut(A::Item) -> O + 'a,
    {
        Box::new(a.into_iter().map(f))
    }

    pub fn pure<'a, A: 'a>(a: A) -> Box<dyn Iterator<Item = A> + 'a> {
        Box::new(once(a))
    }

    pub fn zip_with<'a, A, B, C, F>(mut f: F, a: A, b: B) -> Box<dyn Iterator<Item = C> + 'a>
    where
        A: IntoIterator,
        B: IntoIterator,
        B::IntoIter: Clone + 'a,
        A::Item: Clone + 'a,
        A::IntoIter: 'a,
        F: FnMut(A::Item, B::Item) -> C + 'a,
    {
        let b = b.into_iter();
        Box::new(
            a.into_iter()
                .flat_map(move |a| b.clone().map(move |b| (a.clone(), b)))
                .map(move |(a, b)| f(a, b)),
        )
    }

    pub fn and_then<'a, A, B, F>(a: A, f: F) -> Box<dyn Iterator<Item = B::Item> + 'a>
    where
        A: IntoIterator,
        B: IntoIterator + 'a,
        A::IntoIter: 'a,
        F: FnMut(A::Item) -> B + 'a,
    {
        Box::new(a.into_iter().flat_map(f))
    }

    pub fn fail<'a, T: 'a>(_: &str) -> Box<dyn Iterator<Item = T> + 'a> {
        Self::empty()
    }

    pub fn join<A, B>(a: A) -> Flatten<A::IntoIter>
    where
        A: IntoIterator,
        B: IntoIterator,
        A::Item: IntoIterator<Item = B::Item>,
    {
        a.into_iter().flatten()
    }

    pub fn guard(cond: bool) -> Box<dyn Iterator<Item = ()>> {
        if cond {
            Box::new(once(()))
        } else {
            Box::new(empty())
        }
    }

    pub fn empty<'a, T: 'a>() -> Box<dyn Iterator<Item = T> + 'a> {
        Box::new(empty())
    }

    pub fn choice<'a, L, R>(l: L, r: R) -> Box<dyn Iterator<Item = L::Item> + 'a>
    where
        L: IntoIterator + 'a,
        R: IntoIterator<Item = L::Item> + 'a,
    {
        Box::new(l.into_iter().chain(r))
    }
}

#[cfg(test)]
mod tests {
    use super::super::qdo;
    use super::*;

    #[test]
    fn test_iter_undet_01() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let answer = {
            let a = a.clone();
            let b = b.clone();
            qdo! { Iter {
                x <- a;
                y <- b;
                let z = 100i64;
                return x + y + z
            }}
            .collect::<Vec<_>>()
        };
        let c = a
            .into_iter()
            .flat_map(|x| b.iter().cloned().map(move |y| x + y + 100))
            .collect::<Vec<_>>();
        assert_eq!(answer, c);
    }

    #[test]
    fn test_iter_undet_pattern_01() {
        use either::Either::*;
        let a = vec![Some(1), None, Some(3)];
        let b = vec![Left(4), Left(5), Right(6)];
        let answer = {
            let a = a.clone();
            let b = b.clone();
            qdo! { Iter {
                Some(x) <- a;
                Left(y) <- b.clone();
                let z = 100i64;
                return x + y + z
            }}
            .collect::<Vec<_>>()
        };
        let c = a
            .into_iter()
            .flatten()
            .flat_map(|x| {
                b.iter()
                    .cloned()
                    .flat_map(|x| x.left())
                    .map(move |y| x + y + 100)
            })
            .collect::<Vec<_>>();
        assert_eq!(answer, c);
    }

    #[test]
    fn test_iter_guard() {
        use either::*;
        let is: Vec<Option<i64>> = vec![Some(1), None, Some(3)];
        let js: Vec<Either<i64, i64>> = vec![Left(4), Right(5), Right(6)];
        let ans: Vec<i64> = {
            let is = is.clone();
            let js = js.clone();
            qdo! {Iter {
                Some(i) <- is;
                Right(j) <- js.clone();
                guard j % 2 == 0;
                let k = 100i64;
                return i + j + k
            }}
            .collect()
        };
        let expected = is
            .into_iter()
            .flatten()
            .flat_map(|i| {
                js.iter().cloned().flat_map(move |j| {
                    j.right()
                        .and_then(move |j| (j % 2 == 0).then_some(i + j + 100))
                })
            })
            .collect::<Vec<_>>();
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_zipiter_no_guard() {
        use std::collections::HashSet;
        let is: Vec<i64> = vec![1, 2, 3];
        let js: Vec<i64> = vec![4, 5, 6];
        let ans: HashSet<i64> = {
            let is = is.clone();
            let js = js.clone();
            qdo! {ZipIter {
                i <- is;
                j <- js;
                let k = 100i64;
                return i + j + k
            }}
            .collect()
        };
        let expected = is
            .into_iter()
            .zip(js)
            .map(|(i, j)| i + j + 100)
            .collect::<HashSet<_>>();
        assert_eq!(ans, expected);
    }
}
