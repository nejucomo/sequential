use either::Either;

/// A [SeqGen] produces a sequence of `Output` values or a `Terminal`
///
/// Implementors only need to provide [SeqGen::into_next].
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