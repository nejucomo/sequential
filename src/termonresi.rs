use std::{marker::PhantomData, ops::Try};

use either::Either::{self, Left, Right};

use crate::{Sequential, TransformNext};

/// A [Sequential] which terminates with the first item residual encountered
#[derive(Copy, Clone, Debug)]
pub struct TerminateOnResidual<S, T, I, R>
where
    S: Sequential,
    S::Item: Try<Residual = R>,
    T: Try<Output = S::Terminal, Residual = R>,
{
    seq: S,
    phantom: PhantomData<(T, I, R)>,
}

impl<S, T, I, R> From<S> for TerminateOnResidual<S, T, I, R>
where
    S: Sequential,
    S::Item: Try<Residual = R>,
    T: Try<Output = S::Terminal, Residual = R>,
{
    fn from(seq: S) -> Self {
        TerminateOnResidual {
            seq,
            phantom: PhantomData,
        }
    }
}

impl<S, T, I, R> Sequential for TerminateOnResidual<S, T, I, R>
where
    S: Sequential,
    S::Item: Try<Output = I, Residual = R>,
    T: Try<Output = S::Terminal, Residual = R>,
{
    type Item = I;
    type Terminal = T;

    fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal> {
        self.seq
            .into_next()
            .map_state(TerminateOnResidual::from)
            .map_terminal(T::from_output)
            .and_then(|itemtry| {
                use std::ops::ControlFlow::*;

                match itemtry.branch() {
                    Continue(item) => Left(item),
                    Break(res) => Right(T::from_residual(res)),
                }
            })
    }
}
