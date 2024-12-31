use either::Either::*;
use qualified_do::*;

fn main() {
    use functo_rs::nonlinear::*;
    use qualified_do_macro::qdo;
    let is = vec![1, 2, 3];
    let js = vec![4, 5, 6];

    let ans: Vec<i64> = {
        let is = is.clone();
        let js = js.clone();
        qdo! {UndetVec {
            i <- is.clone();
            j <- js.clone();
            let k = 100i64;
            guard i % 2 == 1;
            return i + j + k
        }}
    };
    assert_eq!(
        ans,
        is.into_iter()
            .flat_map(|i| js.iter().cloned().flat_map(move |j| if i % 2 == 1 {
                Some(i + j + 100)
            } else {
                None
            }))
            .collect::<Vec<_>>()
    );
}
