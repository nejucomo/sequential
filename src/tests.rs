use crate::Sequential;
use test_case::test_case;

#[test_case(0..3 => () ; "iter-to-sequential-via-blanket")]
#[test_case((0..1).and_then(1..3) => ((), ()) ; "iter-to-sequential-via-blanket-and-then")]
fn sequential_0_dot_dot_3<S, T>(s0: S) -> T
where
    S: Sequential<Output = u32, Terminal = T>,
{
    let (s1, n0) = s0.into_next().left().unwrap();
    let (s2, n1) = s1.into_next().left().unwrap();
    let (s3, n2) = s2.into_next().left().unwrap();
    let term = s3.into_next().right().unwrap();
    assert_eq!([0, 1, 2], [n0, n1, n2]);
    term
}

#[test]
fn from_fn_mut() {
    use either::Either::{self, *};

    let mut acc = 0;
    let mut inc = 0;

    let f = || -> Either<u32, String> {
        inc += 1;
        acc += inc;
        if acc <= 10 {
            Left(acc)
        } else {
            Right(format!("overflow: {acc} > 10"))
        }
    };

    let s0 = crate::from_fn_mut(f);
    let (s1, n) = s0.into_next().left().unwrap();
    assert_eq!(1, n);
    let (s2, n) = s1.into_next().left().unwrap();
    assert_eq!(3, n);
    let (s3, n) = s2.into_next().left().unwrap();
    assert_eq!(6, n);
    let (s4, n) = s3.into_next().left().unwrap();
    assert_eq!(10, n);
    let errmsg = s4.into_next().right().unwrap();
    assert_eq!("overflow: 15 > 10", &errmsg);
}
