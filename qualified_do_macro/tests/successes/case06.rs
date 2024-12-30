fn main() {
    use either::Either::*;
    use functo_rs::nonlinear::*;
    use qualified_do_macro::qdo;
    let is = vec![Left(1), Right(2), Left(3)];
    let js = vec![Some(4), Some(5), None];

    let ans: Vec<i64> = {
        let is = is.clone();
        let js = js.clone();
        qdo! {UndetVec {
            Right(i) <- is;
            Some(j) <- js.clone();
            let k = 100i64;
            return i + j + k
        }}
    };
    assert_eq!(
        ans,
        is.into_iter()
            .filter_map(|i| i.right())
            .flat_map(|i| js.iter().flatten().map(move |j| i + j + 100))
            .collect::<Vec<_>>()
    );
}
