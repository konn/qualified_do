fn main() {
    use functo_rs::data::*;
    use qualified_do::qdo;

    let _: Vec<i64> = qdo! {ZipVec {
        x <- vec![1,2,3];
        y <- vec![4, 6];
        return x + y
    }};
}
