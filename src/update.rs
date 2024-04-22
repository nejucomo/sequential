use either::Either::{self, Left, Right};

#[cfg(doc)]
use crate::Sequential;

/// [Sequential::into_next] produces this update
#[derive(Debug)]
pub enum Update<S, I, T> {
    /// The sequence produced an item, `I`, and [Sequential] continuation state, `S`
    Next(S, I),

    /// The sequence terminated with `T`
    Terminate(T),
}
use Update::*;

impl<S, I, T> Update<S, I, T> {
    /// Convert into an [Either]
    pub fn either(self) -> Either<(S, I), T> {
        match self {
            Next(s, i) => Left((s, i)),
            Terminate(t) => Right(t),
        }
    }

    /// Map the state of a [Sequential::into_next](crate::Sequential::into_next) type
    pub fn map_state<F, S2>(self, f: F) -> Update<S2, I, T>
    where
        F: FnOnce(S) -> S2,
    {
        match self {
            Next(s, i) => Next(f(s), i),
            Terminate(t) => Terminate(t),
        }
    }

    /// Map the [Sequential::Item](crate::Sequential::Item) of a [Sequential::into_next](crate::Sequential::into_next) type
    pub fn map_item<F, I2>(self, f: F) -> Update<S, I2, T>
    where
        F: FnOnce(I) -> I2,
    {
        match self {
            Next(s, i) => Next(s, f(i)),
            Terminate(t) => Terminate(t),
        }
    }

    /// Map the [Sequential::Terminal](crate::Sequential::Terminal) of a [Sequential::into_next](crate::Sequential::into_next) type
    pub fn map_terminal<F, T2>(self, f: F) -> Update<S, I, T2>
    where
        F: FnOnce(T) -> T2,
    {
        match self {
            Next(s, i) => Next(s, i),
            Terminate(t) => Terminate(f(t)),
        }
    }

    /// Transform the item into a new item or a terminal
    pub fn and_then<F, I2>(self, f: F) -> Update<S, I2, T>
    where
        F: FnOnce(I) -> Either<I2, T>,
    {
        match self {
            Next(s, i) => match f(i) {
                Left(i2) => Next(s, i2),
                Right(t) => Terminate(t),
            },
            Terminate(t) => Terminate(t),
        }
    }
}

impl<S, I, T> From<Update<S, I, T>> for Either<(S, I), T> {
    fn from(value: Update<S, I, T>) -> Self {
        value.either()
    }
}

impl<S, I, T> From<Either<(S, I), T>> for Update<S, I, T> {
    fn from(either: Either<(S, I), T>) -> Self {
        match either {
            Left((s, i)) => Next(s, i),
            Right(t) => Terminate(t),
        }
    }
}
