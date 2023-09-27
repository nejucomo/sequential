use crate::Sequential;

pub trait IntoSequential<I> {
    type Output;
    type Terminal;
    type Seq: Sequential<I>;

    fn into_sequential(self) -> Self::Seq;
}

impl<S, I> IntoSequential<I> for S
where
    S: Sequential<I>,
{
    type Output = <S as Sequential<I>>::Output;
    type Terminal = <S as Sequential<I>>::Terminal;
    type Seq = S;

    fn into_sequential(self) -> Self::Seq {
        self
    }
}
