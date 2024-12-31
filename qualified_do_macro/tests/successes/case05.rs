fn main() {
    use functo_rs::data::*;
    use qualified_do_macro::qdo;
    let is = vec![1, 2, 3];
    let js = vec![4, 5, 6];

    let ans: Vec<i64> = {
        let is = is.clone();
        let js = js.clone();
        qdo! {ZipVec {
            i <- is;
            j <- js;
            let k = 100i64;
            return i + j + k
        }}
    };
    assert_eq!(
        ans,
        is.into_iter()
            .zip(js)
            .map(|(i, j)| i + j + 100)
            .collect::<Vec<_>>()
    );
}
