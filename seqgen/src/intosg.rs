use crate::SeqGen;

/// Types which can be converted into a [SeqGen] process with specific output and termination types
///
/// A blanked implementation ensures all [SeqGen] types provide [IntoSeqGen], analogous to [Iterator] and [IntoIterator].
pub trait IntoSeqGen {
    /// The output of [IntoSeqGen::IntoSeqGen]
    type Output;
    /// The terminal of [IntoSeqGen::IntoSeqGen]
    type Terminal;
    /// The [SeqGen] value `self` converts into
    type IntoSeqGen: SeqGen<Output = Self::Output, Terminal = Self::Terminal>;

    /// Convert `self` into a [SeqGen] type
    fn into_sequential(self) -> Self::IntoSeqGen;
}

impl<S> IntoSeqGen for S
where
    S: SeqGen,
{
    type Output = <S as SeqGen>::Output;
    type Terminal = <S as SeqGen>::Terminal;
    type IntoSeqGen = S;

    fn into_sequential(self) -> Self::IntoSeqGen {
        self
    }
}
