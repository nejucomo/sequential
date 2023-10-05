use crate::Sequential;
use either::Either::{self, *};

/// Compose a pair of [Sequential] values in sequence, processing all of `U` and then all of `D`
///
/// The uptream (type `U`) processes all inputs to produce associated outputs first, then, once terminated, downstream processes the remaining inputs until terminated.
///
/// The [Sequential::Terminal] value of `U` is held until the entire [AndThen] terminates with both constituent terminals.
pub struct AndThen<I, U, D>(Inner<I, U, D>)
where
    U: Sequential<I>;

enum Inner<I, U, D>
where
    U: Sequential<I>,
{
    UpReady(U, D),
    DownReady(<U as Sequential<I>>::Terminal, D),
}
use Inner::*;

impl<I, U, D> AndThen<I, U, D>
where
    U: Sequential<I>,
{
    pub(super) fn new(upstream: U, downstream: D) -> Self {
        AndThen(UpReady(upstream, downstream))
    }
}

impl<I, U, D, O> Sequential<I> for AndThen<I, U, D>
where
    U: Sequential<I, Output = O>,
    D: Sequential<I, Output = O>,
{
    type Output = O;
    type Terminal = (
        <U as Sequential<I>>::Terminal,
        <D as Sequential<I>>::Terminal,
    );

    fn into_next_with(self, input: I) -> Either<(Self, Self::Output), (Self::Terminal, I)> {
        match self {
            AndThen(UpReady(up, down)) => match up.into_next_with(input) {
                Left((up_new, item)) => Left((AndThen(UpReady(up_new, down)), item)),

                // API BUG: `into_next_with` swallows the input
                Right((up_term, input)) => AndThen(DownReady(up_term, down)).into_next_with(input),
            },
            AndThen(DownReady(up_term, down)) => match down.into_next_with(input) {
                Left((down_new, item)) => Left((AndThen(DownReady(up_term, down_new)), item)),
                Right((down_term, input)) => Right(((up_term, down_term), input)),
            },
        }
    }
}
