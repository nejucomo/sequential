use crate::Sequential;
use either::Either;

/// A [Sequential] transformer that maps each [Sequential::Output] of its inner value.
pub struct MapOutput<S, F, P>
where
    S: Sequential,
    F: Fn(S::Output) -> P,
{
    seq: S,
    f: F,
}

impl<S, F, P> MapOutput<S, F, P>
where
    S: Sequential,
    F: Fn(S::Output) -> P,
{
    pub(crate) fn new(seq: S, f: F) -> Self {
        MapOutput { seq, f }
    }
}

impl<S, F, P> Sequential for MapOutput<S, F, P>
where
    S: Sequential,
    F: Fn(S::Output) -> P,
{
    type Output = P;
    type Terminal = S::Terminal;

    fn into_next(self) -> Either<(Self, Self::Output), Self::Terminal> {
        use crate::TransformNext;

        let MapOutput { seq, f } = self;
        seq.into_next()
            .map_output(&f)
            .map_state(|next| MapOutput::new(next, f))
    }
}
