//! The [Sequential] trait and supporting types for abstract sequential emission of items with explicit termination

use std::ops::Try;

use crate::{AndThen, MapItems, MapTerminal, TerminateOnResidual};
use either::Either;

/// A [Sequential] produces a sequence of [Item](Sequential::Item) values or a [Terminal](Sequential::Terminal)
///
/// Implementors only need to provide [Sequential::into_next].
pub trait Sequential: Sized {
    /// Each non-terminal step of a sequence produces an `Item`
    type Item;
    /// A `Terminal` is produced when a sequence terminates
    type Terminal;

    /// Consume the [Sequential] to produce either a continuation (type `Self`) with an [Item](Sequential::Item) or else a [Terminal](Sequential::Terminal)
    ///
    /// This uses move semantics (consuming the [Sequential] and potentially producing a new one) to ensure in the case of termination, no inconsistent state remains. This also ensures consuming code cannot "iterate past the end" of a sequence.
    fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal>;

    /// After completing `self`, continue with `downstream`, collecting the two terminals into a pair
    fn and_then<D>(self, downstream: D) -> AndThen<Self, D>
    where
        D: Sequential<Item = Self::Item>,
    {
        AndThen::new(self, downstream)
    }

    /// Process each item with `f`, then return [Self::Terminal]
    fn for_each<F>(self, mut f: F) -> Self::Terminal
    where
        F: FnMut(Self::Item),
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
        F: FnMut(Self::Item) -> bool,
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

    /// Map each [Self::Item] another type
    fn map_items<F, P>(self, f: F) -> MapItems<Self, F, P>
    where
        F: Fn(Self::Item) -> P,
    {
        MapItems::new(self, f)
    }

    /// Map the [Self::Terminal] another type
    fn map_terminal<F, P>(self, f: F) -> MapTerminal<Self, F, P>
    where
        F: Fn(Self::Terminal) -> P,
    {
        MapTerminal::new(self, f)
    }

    /// Transform to a [Sequential] which terminates on the first [Try::Residual] (such as [Err]); if no inner item is a residual, terminate with the original [Self::Terminal] as [Try::Output]
    ///
    /// # Example
    ///
    fn terminate_on_residual<T, E>(self) -> TerminateOnResidual<Self, T, E>
    where
        Self::Item: Try<Residual = Result<T, E>>,
    {
        TerminateOnResidual::from(self)
    }
}

impl<I> Sequential for I
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;
    type Terminal = ();

    fn into_next(mut self) -> Either<(Self, Self::Item), Self::Terminal> {
        use Either::*;

        self.next().map(|x| Left((self, x))).unwrap_or(Right(()))
    }
}
