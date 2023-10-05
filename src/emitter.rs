//! The [Emitter] trait and supporting types for abstract sequential emission of outputs with explicit termination

mod andthen;
mod intoemitter;

pub use self::andthen::AndThen;
pub use self::intoemitter::IntoEmitter;

use either::Either;

/// An [Emitter] produces a sequence of `Output` values or a `Terminal`
///
/// Implementors only need to provide [Emitter::into_next].
pub trait Emitter: Sized {
    /// Each non-terminal step of a sequence produces this value
    type Output;
    /// This value is produced when a sequence terminates
    type Terminal;

    /// Consume the [Emitter] to produce either a continuation (type `Self`) with an `Output` or else a `Termination` value.
    ///
    /// This uses move semantics (consuming the [Emitter] and potentially producing a new one) to ensure in the case of termination, no inconsistent emitter state remains.
    fn into_next(self) -> Either<(Self, Self::Output), Self::Terminal>;

    /// After completing `self`, continue with `downstream`, collecting the two terminals into a pair
    fn and_then<D>(self, downstream: D) -> AndThen<Self, D>
    where
        D: Emitter<Output = Self::Output>,
    {
        AndThen::new(self, downstream)
    }
}

impl<I> Emitter for I
where
    I: Iterator,
{
    type Output = <I as Iterator>::Item;
    type Terminal = ();

    fn into_next(mut self) -> Either<(Self, Self::Output), Self::Terminal> {
        use Either::*;

        self.next().map(|x| Left((self, x))).unwrap_or(Right(()))
    }
}
