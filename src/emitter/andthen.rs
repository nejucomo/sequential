use crate::Emitter;
use either::Either::{self, *};

/// Compose a pair of [Emitter] values in sequence, producing all of `U`'s outputs and then all of `D`'s
///
/// The [Emitter::Terminal] value of `U` is held until the entire [AndThen] terminates with both constituent terminals.
pub struct AndThen<U, D>
where
    U: Emitter,
{
    upstate: Either<U, <U as Emitter>::Terminal>,
    down: D,
}

impl<U, D> AndThen<U, D>
where
    U: Emitter,
{
    pub(super) fn new(upstream: U, downstream: D) -> Self {
        AndThen {
            upstate: Left(upstream),
            down: downstream,
        }
    }
}

impl<U, D, O> Emitter for AndThen<U, D>
where
    U: Emitter<Output = O>,
    D: Emitter<Output = O>,
{
    type Output = O;
    type Terminal = (<U as Emitter>::Terminal, <D as Emitter>::Terminal);

    fn into_next(self) -> Either<(Self, Self::Output), Self::Terminal> {
        let AndThen { upstate, down } = self;
        match upstate {
            Left(up) => match up.into_next() {
                Left((up_new, output)) => Left((
                    AndThen {
                        upstate: Left(up_new),
                        down,
                    },
                    output,
                )),
                Right(up_term) => AndThen {
                    upstate: Right(up_term),
                    down,
                }
                .into_next(),
            },
            Right(up_term) => match down.into_next() {
                Left((down_new, output)) => Left((
                    AndThen {
                        upstate: Right(up_term),
                        down: down_new,
                    },
                    output,
                )),
                Right(down_term) => Right((up_term, down_term)),
            },
        }
    }
}
