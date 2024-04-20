use crate::Sequential;
use either::Either;

/// A [Sequential] transformer that maps each [Sequential::Item] of its inner value.
pub struct MapItem<S, F, P>
where
    S: Sequential,
    F: Fn(S::Item) -> P,
{
    seq: S,
    f: F,
}

impl<S, F, P> MapItem<S, F, P>
where
    S: Sequential,
    F: Fn(S::Item) -> P,
{
    pub(crate) fn new(seq: S, f: F) -> Self {
        MapItem { seq, f }
    }
}

impl<S, F, P> Sequential for MapItem<S, F, P>
where
    S: Sequential,
    F: Fn(S::Item) -> P,
{
    type Item = P;
    type Terminal = S::Terminal;

    fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal> {
        use crate::TransformNext;

        let MapItem { seq, f } = self;
        seq.into_next()
            .map_output(&f)
            .map_state(|next| MapItem::new(next, f))
    }
}
