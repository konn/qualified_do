fn main() {
    use functo_rs::data::*;
    use functo_rs::impls::*;
    use qualified_do::qdo;
    let is = vec![1, 2, 3];
    let js = vec![4, 5, 6];

    let ans: Vec<i64> = qdo! {ZipVec {
        i <- is.clone();
        j <- js.clone();
        let k = 100i64;
        return i + j + k
    }};
    assert_eq!(
        ans,
        is.into_iter()
            .zip(js)
            .map(|(i, j)| i + j + 100)
            .collect::<Vec<_>>()
    );
}
