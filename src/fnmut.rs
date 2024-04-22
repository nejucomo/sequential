use crate::Sequential;
use crate::Update::{self, Next, Terminate};
use either::Either;

/// Wraps a [FnMut] which returns [Sequential] items or a terminal
pub struct SequentialFnMut<F>(F);

/// Wrap a [FnMut] which returns [Sequential] items or a terminal
pub fn from_fn_mut<F, O, T>(f: F) -> SequentialFnMut<F>
where
    F: FnMut() -> Either<O, T>,
{
    SequentialFnMut(f)
}

impl<F, O, T> Sequential for SequentialFnMut<F>
where
    F: FnMut() -> Either<O, T>,
{
    type Item = O;
    type Terminal = T;

    fn into_next(mut self) -> Update<Self, Self::Item, Self::Terminal> {
        self.0()
            .map_left(|item| Next(self, item))
            .map_right(Terminate)
            .into_inner()
    }
}
