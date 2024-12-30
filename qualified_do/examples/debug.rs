use either::Either::*;
use qualified_do::*;

fn main() {
    let a = vec![Some(1), None, Some(3)];
    let b = vec![Left(4), Left(5), Right(6)];
    let answer = {
        let a = a.clone();
        let b = b.clone();
        qdo! { Iter {
            Some(x) <- a;
            Left(y) <- b.clone();
            let z = 100i64;
            return x + y + z
        }}
        .collect::<Vec<_>>()
    };
    let c = a
        .into_iter()
        .flatten()
        .flat_map(|x| {
            b.iter()
                .cloned()
                .flat_map(|x| x.left())
                .map(move |y| x + y + 100)
        })
        .collect::<Vec<_>>();
    assert_eq!(answer, c);
}
