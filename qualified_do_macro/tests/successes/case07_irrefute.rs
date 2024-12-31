fn main() {
    use functo_rs::nonlinear::*;
    use qualified_do_macro::qdo;
    let is = vec![1, 2, 3];
    let js = vec![(4,), (5,), (6,)];

    let ans: Vec<i64> = {
        let is = is.clone();
        let js = js.clone();
        qdo! {UndetVec {
            i <- is;
            ~(j,) <- js.clone();
            let k = 100i64;
            return i + j + k
        }}
    };
    assert_eq!(
        ans,
        is.into_iter()
            .flat_map(|i| js.iter().map(|(a,)| a).map(move |j| i + j + 100))
            .collect::<Vec<_>>()
    );
}
