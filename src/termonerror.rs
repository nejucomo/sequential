use crate::Sequential;
use either::Either::{self, *};

/// Convert a [Sequential] that generates [Result] outputs to one which terminates on the first [Err] output, or else with [Ok] on successful completion
pub fn terminate_on_error<T, E>(
    errorseq: impl Sequential<Output = Result<T, E>, Terminal = ()>,
) -> impl Sequential<Output = T, Terminal = Result<(), E>> {
    TerminateOnError(errorseq)
}

/// See [terminate_on_error] which constructs this type
#[derive(Copy, Clone, Debug)]
pub struct TerminateOnError<S, T, E>(S)
where
    S: Sequential<Output = Result<T, E>, Terminal = ()>;

impl<S, T, E> Sequential for TerminateOnError<S, T, E>
where
    S: Sequential<Output = Result<T, E>, Terminal = ()>,
{
    type Output = T;
    type Terminal = Result<(), E>;

    fn into_next(self) -> Either<(Self, Self::Output), Self::Terminal> {
        match self.0.into_next() {
            Left((next, Ok(item))) => Left((TerminateOnError(next), item)),
            Left((_, Err(e))) => Right(Err(e)),
            Right(()) => Right(Ok(())),
        }
    }
}
