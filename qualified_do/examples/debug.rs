fn main() {
    use syn::*;

    println!(
        "{:?}",
        syn::parse_str::<Expr>("match hoge { ::None => true }").unwrap()
    );
}
