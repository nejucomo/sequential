//! The [Sequential] trait and supporting types for abstract sequential emission of outputs with explicit termination

use crate::{AndThen, MapOutput, MapTerminal};
use either::Either;

/// A [Sequential] produces a sequence of `Output` values or a `Terminal`
///
/// Implementors only need to provide [Sequential::into_next].
pub trait Sequential: Sized {
    /// Each non-terminal step of a sequence produces this value
    type Output;
    /// This value is produced when a sequence terminates
    type Terminal;

    /// Consume the [Sequential] to produce either a continuation (type `Self`) with an `Output` or else a `Termination` value.
    ///
    /// This uses move semantics (consuming the [Sequential] and potentially producing a new one) to ensure in the case of termination, no inconsistent emitter state remains.
    fn into_next(self) -> Either<(Self, Self::Output), Self::Terminal>;

    /// After completing `self`, continue with `downstream`, collecting the two terminals into a pair
    fn and_then<D>(self, downstream: D) -> AndThen<Self, D>
    where
        D: Sequential<Output = Self::Output>,
    {
        AndThen::new(self, downstream)
    }

    /// Process each item with `f`, then return [Self::Terminal]
    fn for_each<F>(self, mut f: F) -> Self::Terminal
    where
        F: FnMut(Self::Output),
    {
        use either::Either::*;

        let mut seq = self;
        loop {
            match seq.into_next() {
                Left((next, item)) => {
                    f(item);
                    seq = next;
                }
                Right(term) => {
                    return term;
                }
            }
        }
    }

    /// Process items with `f` as long as it returns [true]
    ///
    /// Either the remainder of the pending sequence, [Self], is returned, or else if it completed, [Self::Terminal].
    ///
    /// # Example: Terminate on Break
    ///
    /// If a caller wishes to process some initial items, drop the rest, and collect the terminal, this is a concise approach:
    ///
    /// ```
    /// # use sequential::Sequential;
    /// # fn process_item<T>(_: T) -> bool { true }
    /// fn process_sequential<S>(seq: S) -> <S as Sequential>::Terminal
    /// where
    ///     S: Sequential,
    /// {
    ///      seq.for_each_or_break(process_item).map_left(Sequential::terminate).into_inner()
    /// }
    /// ```
    fn for_each_or_break<F>(self, mut f: F) -> Either<Self, Self::Terminal>
    where
        F: FnMut(Self::Output) -> bool,
    {
        use either::Either::*;

        let mut seq = self;
        loop {
            match seq.into_next() {
                Left((next, item)) => {
                    if !f(item) {
                        return Left(next);
                    }
                    seq = next;
                }
                Right(term) => {
                    return Right(term);
                }
            }
        }
    }

    /// Drop all items to return [Self::Terminal]
    fn terminate(self) -> Self::Terminal {
        self.for_each(std::mem::drop)
    }

    /// Map each [Self::Output] another type
    fn map_output<F, P>(self, f: F) -> MapOutput<Self, F, P>
    where
        F: Fn(Self::Output) -> P,
    {
        MapOutput::new(self, f)
    }

    /// Map the [Self::Terminal] another type
    fn map_terminal<F, P>(self, f: F) -> MapTerminal<Self, F, P>
    where
        F: Fn(Self::Terminal) -> P,
    {
        MapTerminal::new(self, f)
    }
}

impl<I> Sequential for I
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
