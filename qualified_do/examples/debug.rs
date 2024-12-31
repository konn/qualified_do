fn main() {
    use functo_rs::nonlinear::*;
    use qualified_do::qdo;
    let j = 1 + 1;
    let _: Vec<i64> = qdo! {UndetVec {
        x <- vec![1,2,3];
        y <- vec![4, 6];
        guard j % 2 == 0;
        return x + y
    }};
}
