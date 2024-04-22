use std::{convert::Infallible, marker::PhantomData, ops::Try};

use crate::{Sequential, Update};

/// A [Sequential] which terminates with the first item residual encountered
#[derive(Copy, Clone, Debug)]
pub struct TerminateOnErr<S, E> {
    seq: S,
    phantom: PhantomData<E>,
}

impl<S, E> From<S> for TerminateOnErr<S, E> {
    fn from(seq: S) -> Self {
        TerminateOnErr {
            seq,
            phantom: PhantomData,
        }
    }
}

impl<S, E> Sequential for TerminateOnErr<S, E>
where
    S: Sequential,
    S::Item: Try<Residual = Result<Infallible, E>>,
{
    type Item = <S::Item as Try>::Output;
    type Terminal = Result<S::Terminal, E>;

    fn into_next(self) -> Update<Self, Self::Item, Self::Terminal> {
        self.seq
            .into_next()
            .map_state(TerminateOnErr::from)
            .map_item(|r| r.branch())
            .terminate_on_break()
    }
}
