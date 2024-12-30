fn main() {
    use functo_rs::control::*;
    use functo_rs::impls::*;
    use qualified_do::qdo;

    let ans: Option<()> = qdo! {OptionFunctor {
        i <- Some(5);
        j <- Some(6);
        let k = 7i64;
        return i + j + k;
    }};
    assert_eq!(ans, Some(()));
}
