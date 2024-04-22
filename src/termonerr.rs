use std::{marker::PhantomData, ops::Try};

use either::Either::{self, Left, Right};

use crate::{Sequential, TransformNext};

/// A [Sequential] which terminates with the first item residual encountered
#[derive(Copy, Clone, Debug)]
pub struct TerminateOnErr<S, T, E> {
    seq: S,
    phantom: PhantomData<(T, E)>,
}

impl<S, T, E> From<S> for TerminateOnErr<S, T, E> {
    fn from(seq: S) -> Self {
        TerminateOnErr {
            seq,
            phantom: PhantomData,
        }
    }
}

impl<S, T, E> Sequential for TerminateOnErr<S, T, E>
where
    S: Sequential,
    S::Item: Try<Residual = Result<T, E>>,
{
    type Item = <S::Item as Try>::Output;
    type Terminal = Result<S::Terminal, E>;

    fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal> {
        self.seq
            .into_next()
            .map_state(TerminateOnErr::from)
            .map_terminal(Ok)
            .and_then(|itemtry| {
                use std::ops::ControlFlow::*;

                match itemtry.branch() {
                    Continue(item) => Left(item),
                    Break(Err(res)) => Right(Err(res)),
                    Break(Ok(_)) => unreachable!("infallible Result residual Ok"),
                }
            })
    }
}
