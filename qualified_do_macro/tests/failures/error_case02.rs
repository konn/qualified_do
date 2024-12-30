fn main() {
    use qualified_do_macro::qdo;

    qdo! {hoge::fuga {
        return 4;
        x <- Some(5)
    }}
}
