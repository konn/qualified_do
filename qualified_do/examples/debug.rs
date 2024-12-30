fn main() {
    use functo_rs::control::*;
    use functo_rs::impls::*;
    use qualified_do::qdo;

    let _: Option<()> = qdo! {OptionFunctor {
        let k = 7i64;
        return k;
    }};
}
