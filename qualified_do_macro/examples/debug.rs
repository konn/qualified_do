fn main() {
    use functo_rs::data::*;
    use qualified_do_macro::qdo;

    let _ = qdo! {AsData::<ZipVec> {
        x <- vec![1,2,3];
        y <- vec![4,5,6];
        return x + y
    }};
}
