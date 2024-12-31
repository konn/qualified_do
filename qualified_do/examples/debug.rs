fn main() {
    use either::Either;
    use either::Either::*;
    use qualified_do::{qdo, Iter};
    let ans: Vec<i64> = {
        let is: Vec<Option<i64>> = vec![Some(1), None, Some(3)];
        let js: Vec<Either<i64, i64>> = vec![Left(4), Right(5), Right(6)];
        qdo! {Iter {
            Some(i) <- is.clone();
            Right(j) <- js.clone();
            guard j % 2 == 0;
            let k = 100i64;
            return i + j + k
        }}
        .collect()
    };
    assert_eq!(ans, vec![107, 109]);
}
