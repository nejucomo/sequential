use crate::Sequential;

/// Types which can be converted into a [Sequential] process with specific input, output, and termination types
///
/// A blanked implementation ensures all [Sequential] types provide [IntoSequential], analogous to [Iterator] and [IntoIterator].
pub trait IntoSequential<I> {
    type Output;
    type Terminal;
    type Seq: Sequential<I>;

    /// Convert `self` into a [Sequential] type
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
