use either::Either;
use sequential::Sequential;

/// A [SeqGen] type produces a sequence of `Output` values or a `Terminal`
///
/// Implementors only need to provide [SeqGen::into_next].
///
/// The [SeqGen] trait encapsulates a subset of the [Sequential] trait, and any `Sequential<()>` is also a [SeqGen] via a blanket impl.
pub trait SeqGen: Sized {
    /// Each non-terminal step of a sequence produces this value
    type Output;
    /// This value is produced when a sequence terminates
    type Terminal;

    /// Consume the [SeqGen] to produce either a continuation (type `Self`) with an `Output` or else a `Termination` value
    ///
    /// This uses move semantics (consuming the [SeqGen] and potentially producing a new one) to ensure in the case of termination, no inconsistent sequencing state remains.
    fn into_next(self) -> Either<(Self, Self::Output), Self::Terminal>;
}

impl<T> SeqGen for T
where
    T: Sequential<()>,
{
    type Output = <T as Sequential<()>>::Output;
    type Terminal = <T as Sequential<()>>::Terminal;

    fn into_next(self) -> Either<(Self, Self::Output), Self::Terminal> {
        self.into_next_with(())
    }
}
