use std::iter::Iterator;
use std::iter::*;

pub enum ZipIter {}

impl ZipIter {
    pub fn fmap<A, O, F>(f: F, a: A) -> Map<A, F>
    where
        A: Iterator,
        F: FnMut(A::Item) -> O,
    {
        a.map(f)
    }

    pub fn pure<A: Clone>(a: A) -> Repeat<A> {
        repeat(a)
    }

    pub fn zip_with<A, B, C, F>(mut f: F, a: A, b: B) -> impl Iterator<Item = C>
    where
        A: Iterator,
        B: Iterator,
        F: FnMut(A::Item, B::Item) -> C,
    {
        Box::new(a.zip(b).map(move |(a, b)| f(a, b)))
    }
}
pub enum Iter {}

impl Iter {
    pub fn fmap<A, O, F>(f: F, a: A) -> Map<A, F>
    where
        A: Iterator,
        F: FnMut(A::Item) -> O,
    {
        a.map(f)
    }

    pub fn pure<A>(a: A) -> Once<A> {
        once(a)
    }

    pub fn zip_with<A, B, C, F>(mut f: F, a: A, b: B) -> impl Iterator<Item = C>
    where
        A: Iterator,
        B: Iterator + Clone,
        A::Item: Clone,
        F: FnMut(A::Item, B::Item) -> C,
    {
        Box::new(
            a.flat_map(move |a| b.clone().map(move |b| (a.clone(), b)))
                .map(move |(a, b)| f(a, b)),
        )
    }

    pub fn and_then<A, B, F>(f: F, a: A) -> FlatMap<A, B, F>
    where
        A: Iterator,
        B: Iterator,
        F: FnMut(A::Item) -> B,
    {
        a.flat_map(f)
    }
}
