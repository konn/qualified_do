use core::str;
use std::iter::Iterator;
use std::iter::*;

pub enum ZipIter {}

impl ZipIter {
    pub fn fmap<A, O, F>(f: F, a: A) -> Box<dyn Iterator<Item = O>>
    where
        A: IntoIterator + 'static,
        F: FnMut(A::Item) -> O + 'static,
    {
        Box::new(a.into_iter().map(f))
    }

    pub fn pure<A: Clone + 'static>(a: A) -> Box<dyn Iterator<Item = A>> {
        Box::new(repeat(a))
    }

    pub fn zip_with<A, B, C, F>(mut f: F, a: A, b: B) -> Box<dyn Iterator<Item = C>>
    where
        A: IntoIterator + 'static,
        B: IntoIterator + 'static,
        F: FnMut(A::Item, B::Item) -> C + 'static,
    {
        Box::new(a.into_iter().zip(b).map(move |(a, b)| f(a, b)))
    }
}
pub enum Iter {}

impl Iter {
    pub fn fmap<A, O, F>(f: F, a: A) -> Box<dyn Iterator<Item = O>>
    where
        A: IntoIterator + 'static,
        F: FnMut(A::Item) -> O + 'static,
    {
        Box::new(a.into_iter().map(f))
    }

    pub fn pure<A: 'static>(a: A) -> Box<dyn Iterator<Item = A>> {
        Box::new(once(a))
    }

    pub fn zip_with<A, B, C, F>(mut f: F, a: A, b: B) -> Box<dyn Iterator<Item = C>>
    where
        A: IntoIterator,
        B: IntoIterator,
        B::IntoIter: Clone + 'static,
        A::Item: Clone + 'static,
        A::IntoIter: 'static,
        F: FnMut(A::Item, B::Item) -> C + 'static,
    {
        let b = b.into_iter();
        Box::new(
            a.into_iter()
                .flat_map(move |a| b.clone().map(move |b| (a.clone(), b)))
                .map(move |(a, b)| f(a, b)),
        )
    }

    pub fn and_then<A, B, F>(a: A, f: F) -> Box<dyn Iterator<Item = B::Item>>
    where
        A: IntoIterator,
        B: IntoIterator + 'static,
        A::IntoIter: 'static,
        F: FnMut(A::Item) -> B + 'static,
    {
        Box::new(a.into_iter().flat_map(f))
    }

    pub fn fail<T: 'static>(_: &str) -> Box<dyn Iterator<Item = T>> {
        Box::new(empty())
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
}
