fn main() {
    use functo_rs::control::*;
    use qualified_do_macro::qdo;

    let ans: Option<i64> = qdo! {OptionFunctor {
        i <- Some(5);
        j <- Some(6);
        let k = 7i64;
        return i + j + k
    }};
    assert_eq!(ans, Some(18));
}
