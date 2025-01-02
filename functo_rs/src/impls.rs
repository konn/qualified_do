//! Various implementations of functors.

use std::marker::PhantomData;

pub enum Identity {}

pub enum UndetVec {}

pub enum ZipVec {}

pub enum OptionFunctor {}

pub struct ResultFunctor<E> {
    phantom: PhantomData<E>,
}

pub enum V2 {}

pub struct Reader<R> {
    pub env: PhantomData<R>,
}

pub struct State<S> {
    pub env: PhantomData<S>,
}

pub struct ArrayFunctor<const N: usize> {}
