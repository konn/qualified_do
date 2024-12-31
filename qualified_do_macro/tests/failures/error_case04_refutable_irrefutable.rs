fn main() {
    use functo_rs::data::*;
    use qualified_do_macro::qdo;

    qdo! {ZipVec {
        x <- vec![1,2,3];
        ~Some(y) <- vec![Some(4), None, Some(6)];
        return x + y;
    }};
}
