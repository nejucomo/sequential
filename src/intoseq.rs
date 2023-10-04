use crate::Sequential;

/// Types which can be converted into a [Sequential] process with specific input, output, and termination types
///
/// A blanked implementation ensures all [Sequential] types provide [IntoSequential], analogous to [Iterator] and [IntoIterator].
pub trait IntoSequential<I> {
    /// The output of [IntoSequential::Seq]
    type Output;
    /// The terminal of [IntoSequential::Seq]
    type Terminal;
    /// The [Sequential] value `self` converts into
    type Seq: Sequential<I, Output = Self::Output, Terminal = Self::Terminal>;

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
