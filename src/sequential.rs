//! The [Sequential] trait and supporting types for abstract sequential processing over inputs, outputs, and explicit termination

mod intosequential;
mod pipe;

pub use self::intosequential::IntoSequential;
pub use self::pipe::{Pipe, PipeTerminal};

use either::Either;

/// A [Sequential] type processes inputs of type `I`, and produces a sequence of `Output` values or a `Terminal`
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

    /// Pipe `self` outputs into `downstream` inputs to produce a [Sequential] [Pipe] composition
    fn pipe_into<D>(self, downstream: D) -> Pipe<Self, D>
    where
        D: Sequential<Self::Output>,
    {
        Pipe::from_parts(self, downstream)
    }
}
