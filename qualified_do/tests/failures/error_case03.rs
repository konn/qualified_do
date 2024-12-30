fn main() {
    use functo_rs::data::*;
    use qualified_do::qdo;

    qdo! {ZipVec {
        x <- vec![1,2,3];
        y <- vec![4,5,6];
        if x % 2 == 1 { vec![()]} else { vec![] };
        return x + y;
    }}
}
