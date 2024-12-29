//! Various implementations of functors.

pub enum Identity {}

pub enum UndetVec {}

pub enum ZipVec {}

pub enum OptionFunctor {}

pub struct ResultFunctor<E> {
    phantom: std::marker::PhantomData<E>,
}

pub enum V2 {}

pub struct Reader<E> {
    pub env: E,
}
