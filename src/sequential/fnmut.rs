use crate::Sequential;
use either::Either;

/// Wraps a [FnMut] which returns [Sequential] outputs or a terminal
pub struct SequentialFnMut<F>(F);

/// Wrap a [FnMut] which returns [Sequential] outputs or a terminal
pub fn from_fn_mut<F, I, O, T>(f: F) -> SequentialFnMut<F>
where
    F: FnMut(I) -> Either<O, (T, I)>,
{
    SequentialFnMut(f)
}

impl<F, I, O, T> Sequential<I> for SequentialFnMut<F>
where
    F: FnMut(I) -> Either<O, (T, I)>,
{
    type Output = O;
    type Terminal = T;

    fn into_next_with(mut self, input: I) -> Either<(Self, O), (T, I)> {
        self.0(input).map_left(|output| (self, output))
    }
}
