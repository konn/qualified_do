pub use qualified_do_macro::qdo;

pub mod iter;
pub use iter::Iter;
pub use iter::ZipIter;

pub use functo_rs::control::AsControl;
pub use functo_rs::data::AsData;
pub use functo_rs::impls::*;
pub use functo_rs::nonlinear::AsNonlinear;

pub type Optioned = AsControl<OptionFunctor>;
pub type Resulted<E> = AsControl<ResultFunctor<E>>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_optioned_applicative() {
        let answer = qdo! {Optioned {
            x <- Some(1);
            y <- Some(2);
            return x + y + 100
        }};
        assert_eq!(answer, Some(103));
    }
}
