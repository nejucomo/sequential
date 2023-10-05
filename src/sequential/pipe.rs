use crate::Sequential;
use either::Either::{self, *};

/// A composition of upstream (type `U`) and downstream (type `D`) [Sequential] types
///
/// The [Sequential] impl of [Pipe] processes inputs through the upstream, passing the output to the downstream. When either member terminates, that termination value as well as the remaining state of the other member are returned in the [PipeTerminal].
#[derive(Copy, Clone, Debug)]
pub struct Pipe<U, D>(U, D);

/// The [Sequential::Terminal] of a [Pipe] with full state
///
/// A [Pipe] terminates when either of the two constituent [Sequential]s terminate, which leaves the state of the other non-terminated [Sequential]. This type contains both the terminal value of one constituent and the complete state of the other constituent.
///
/// It is possible for callers to use this complete state with [PipeTerminal::unwrap] to continue the unterminated [Sequential]. However, many applications will ignore unterminated constituents and care only about the constituent terminal values, so they can use [PipeTerminal::child_terminal].
pub struct PipeTerminal<I, U, D>(
    Either<
        (<U as Sequential<I>>::Terminal, D),
        (U, <D as Sequential<<U as Sequential<I>>::Output>>::Terminal),
    >,
)
where
    U: Sequential<I>,
    D: Sequential<<U as Sequential<I>>::Output>;

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
    type Terminal = PipeTerminal<I, U, D>;

    fn into_next_with(self, input: I) -> Either<(Self, Self::Output), Self::Terminal> {
        let Pipe(up, down) = self;
        match up.into_next_with(input) {
            Left((next_up, item)) => match down.into_next_with(item) {
                Left((next_down, output)) => Left((Pipe(next_up, next_down), output)),
                Right(down_term) => Right(PipeTerminal(Right((next_up, down_term)))),
            },
            Right(up_term) => Right(PipeTerminal(Left((up_term, down)))),
        }
    }
}

impl<I, U, D> PipeTerminal<I, U, D>
where
    U: Sequential<I>,
    D: Sequential<<U as Sequential<I>>::Output>,
{
    /// Unwrap the full state of [Pipe] termination, including one constituent's [Sequential::Terminal] value and the other constituent's full (unterminated) state
    pub fn unwrap(
        self,
    ) -> Either<
        (<U as Sequential<I>>::Terminal, D),
        (U, <D as Sequential<<U as Sequential<I>>::Output>>::Terminal),
    > {
        self.0
    }

    /// Discard the pending unterminated state of one [Pipe] constituent and simply return the [Sequential::Terminal]
    pub fn child_terminal(
        self,
    ) -> Either<
        <U as Sequential<I>>::Terminal,
        <D as Sequential<<U as Sequential<I>>::Output>>::Terminal,
    > {
        self.0
            .map_left(|(up_term, _)| up_term)
            .map_right(|(_, down_term)| down_term)
    }
}
