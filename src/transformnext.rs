use either::Either;

/// A helper trait for transforming the return values of [Sequential::into_next](crate::Sequential::into_next)
///
/// There is an impl for `Either<(S, O), T>` covering the return value of [Sequential::into_next](crate::Sequential::into_next), and each transformation method results in a type also covered by this impl, enabling chained calls.
///
/// # Example
///
/// Rather than use [Either] methods directly, this crate provides [Sequential](crate::Sequential)-specific convenience methods for transforming results. Consider if we were implementing a [Sequential](crate::Sequential) wrapper which multiplies each [Sequential::Item](crate::Sequential::Item) by 2 and the terminal by 3. Without [TransformNext] this might look like this:
///
/// ```
/// use sequential::Sequential;
/// use either::Either;
///
/// struct Wrapper<S>(S);
///
/// impl<S> Sequential for Wrapper<S>
/// where S:
///     Sequential<Item = i64, Terminal = i64>,
/// {
///     type Item = S::Item;
///     type Terminal = S::Terminal;
///
///     fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal> {
///         self.0.into_next()
///             .map_left(|(next, item)| (Wrapper(next), item * 2))
///             .map_right(|term| term * 3)
///     }
/// }
/// ```
///
/// By using [TransformNext] we can implement this in a more readable ergonomic fashion:
///
/// ```
/// use sequential::{Sequential, TransformNext};
/// use either::Either;
///
/// struct Wrapper<S>(S);
///
/// impl<S> Sequential for Wrapper<S>
/// where S:
///     Sequential<Item = i64, Terminal = i64>,
/// {
///     type Item = S::Item;
///     type Terminal = S::Terminal;
///
///     fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal> {
///         self.0.into_next()
///             .map_state(Wrapper)
///             .map_item(|x| x*2)
///             .map_terminal(|x| x*3)
///     }
/// }
/// ```
pub trait TransformNext<S, O, T> {
    /// Map the state of a [Sequential::into_next](crate::Sequential::into_next) type
    fn map_state<F, S2>(self, f: F) -> Either<(S2, O), T>
    where
        F: FnOnce(S) -> S2;

    /// Map the [Sequential::Item](crate::Sequential::Item) of a [Sequential::into_next](crate::Sequential::into_next) type
    fn map_item<F, O2>(self, f: F) -> Either<(S, O2), T>
    where
        F: FnOnce(O) -> O2;

    /// Map the [Sequential::Terminal](crate::Sequential::Terminal) of a [Sequential::into_next](crate::Sequential::into_next) type
    fn map_terminal<F, T2>(self, f: F) -> Either<(S, O), T2>
    where
        F: FnOnce(T) -> T2;
}

impl<S, O, T> TransformNext<S, O, T> for Either<(S, O), T> {
    fn map_state<F, S2>(self, f: F) -> Either<(S2, O), T>
    where
        F: FnOnce(S) -> S2,
    {
        self.map_left(|(s, o)| (f(s), o))
    }

    fn map_item<F, O2>(self, f: F) -> Either<(S, O2), T>
    where
        F: FnOnce(O) -> O2,
    {
        self.map_left(|(s, o)| (s, f(o)))
    }

    fn map_terminal<F, T2>(self, f: F) -> Either<(S, O), T2>
    where
        F: FnOnce(T) -> T2,
    {
        self.map_right(f)
    }
}
