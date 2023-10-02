use either::Either;

/// A [Sequential] process that processes inputs of type `I`, and produces a sequence of `Output` values or a `Terminal`
///
/// Implementors only need to provide [Sequential::into_next_with].
pub trait Sequential<I>: Sized {
    /// Each non-terminal step of a sequence produces this value
    type Output;
    /// This value is produced when a sequence terminates
    type Terminal;

    /// Consume the [Sequential] and an `input` to produce either a continuation (type `Self`) with an `Output` or else a `Termination` value
    ///
    /// This uses move semantics (consuming the [Sequential] and potentially producing a new one) to ensure in the case of termination, no inconsistent sequencing state remains.
    fn into_next_with(self, input: I) -> Either<(Self, Self::Output), Self::Terminal>;
}
