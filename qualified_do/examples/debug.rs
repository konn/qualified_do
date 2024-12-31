fn main() {
    use functo_rs::nonlinear::*;
    use qualified_do::qdo;

    let _: Vec<i64> = qdo! {UndetVec {
        x <- vec![1,2,3];
        y <- vec![4, 6];
        guard 2 % 2 == 0;
        return x + y
    }};
}
