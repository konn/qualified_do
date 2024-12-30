fn main() {
    use qualified_do::qdo;

    qdo! {hoge::fuga {
        return 4;
        x <- Some(5)
    }}
}
