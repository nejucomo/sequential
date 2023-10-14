use crate::Sequential;
use either::Either;

/// A [Sequential] transformer that maps the [Sequential::Terminal] of its inner value.
pub struct MapTerminal<S, F, U>
where
    S: Sequential,
    F: Fn(S::Terminal) -> U,
{
    seq: S,
    f: F,
}

impl<S, F, U> MapTerminal<S, F, U>
where
    S: Sequential,
    F: Fn(S::Terminal) -> U,
{
    pub(crate) fn new(seq: S, f: F) -> Self {
        MapTerminal { seq, f }
    }
}

impl<S, F, U> Sequential for MapTerminal<S, F, U>
where
    S: Sequential,
    F: Fn(S::Terminal) -> U,
{
    type Output = S::Output;
    type Terminal = U;

    fn into_next(self) -> Either<(Self, Self::Output), Self::Terminal> {
        use crate::TransformNext;

        let MapTerminal { seq, f } = self;
        seq.into_next()
            .map_terminal(&f)
            .map_state(|next| MapTerminal::new(next, f))
    }
}
