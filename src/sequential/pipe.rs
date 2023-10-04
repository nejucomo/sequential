use crate::Sequential;
use either::Either::{self, *};

/// A composition of upstream (type `U`) and downstream (type `D`) [Sequential] types
///
/// The [Sequential] impl of [Pipe] processes inputs through the upstream, passing the output to the downstream. When either member terminates, that termination value as well as the remaining state of the other member are returned in the [Pipe]'s `Terminal`.
#[derive(Copy, Clone, Debug)]
pub struct Pipe<U, D>(U, D);

impl<U, D> Pipe<U, D> {
    /// Construct a [Pipe] from `upstream` and `downstream` [Sequential] values
    pub fn from_parts(upstream: U, downstream: D) -> Self {
        Pipe(upstream, downstream)
    }
}

impl<I, U, D> Sequential<I> for Pipe<U, D>
where
    U: Sequential<I>,
    D: Sequential<<U as Sequential<I>>::Output>,
{
    type Output = <D as Sequential<<U as Sequential<I>>::Output>>::Output;
    type Terminal = Either<
        (<U as Sequential<I>>::Terminal, D),
        (U, <D as Sequential<<U as Sequential<I>>::Output>>::Terminal),
    >;

    fn into_next_with(self, input: I) -> Either<(Self, Self::Output), Self::Terminal> {
        let Pipe(up, down) = self;
        match up.into_next_with(input) {
            Left((next_up, item)) => match down.into_next_with(item) {
                Left((next_down, output)) => Left((Pipe(next_up, next_down), output)),
                Right(down_term) => Right(Right((next_up, down_term))),
            },
            Right(up_term) => Right(Left((up_term, down))),
        }
    }
}
