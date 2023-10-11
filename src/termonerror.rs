use crate::Sequential;
use either::Either::{self, *};

/// Convert a [Sequential] that generates [Result] outputs to one which terminates on the first [Err] output, or else with [Ok] on successful completion
///
/// Note that because there is a blanket impl of [Sequential] for all [Iterator] types, this helps convert the common pattern in [std] where certain [Iterator] types have `Item = Result<T, E>` with the convention that any [Err] item should prevent further iteration. This function translates a _convention_ to a type-safe API, ensuring consumers never iterate after error.
///
/// # Example
///
/// ```
/// use std::io::{BufRead, BufReader, Read};
/// use sequential::Sequential;
/// use either::Either::{Left, Right};
///
/// fn count_lines_and_chars<R>(r: R) -> std::io::Result<(usize, usize)>
/// where
///     R: Read,
/// {
///     let mut lines = 0;
///     let mut chars = 0;
///     let mut seq = sequential::terminate_on_error(BufReader::new(r).lines());
///     loop {
///         match seq.into_next() {
///             Left((next, line)) => {
///                 lines += 1;
///                 chars += line.chars().count();
///                 seq = next;
///             }
///             Right(res) => {
///                 return res.map(|()| (lines, chars));
///             }
///         }
///     }
/// }
/// ```
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
