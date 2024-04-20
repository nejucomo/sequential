use crate::Sequential;
use either::Either::{self, *};

/// Compose a pair of [Sequential] values in sequence, producing all of `U`'s outputs and then all of `D`'s
///
/// The [Sequential::Terminal] value of `U` is held until the entire [AndThen] terminates with both constituent terminals.
pub struct AndThen<U, D>
where
    U: Sequential,
{
    upstate: Either<U, <U as Sequential>::Terminal>,
    down: D,
}

impl<U, D> AndThen<U, D>
where
    U: Sequential,
{
    pub(super) fn new(upstream: U, downstream: D) -> Self {
        AndThen {
            upstate: Left(upstream),
            down: downstream,
        }
    }
}

impl<U, D, O> Sequential for AndThen<U, D>
where
    U: Sequential<Item = O>,
    D: Sequential<Item = O>,
{
    type Item = O;
    type Terminal = (<U as Sequential>::Terminal, <D as Sequential>::Terminal);

    fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal> {
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
