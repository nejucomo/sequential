use crate::{Sequential, Update};

/// Map each [Item](Sequential::Item) of a [Sequential]
pub struct MapItems<S, F, P>
where
    S: Sequential,
    F: Fn(S::Item) -> P,
{
    seq: S,
    f: F,
}

impl<S, F, P> MapItems<S, F, P>
where
    S: Sequential,
    F: Fn(S::Item) -> P,
{
    pub(crate) fn new(seq: S, f: F) -> Self {
        MapItems { seq, f }
    }
}

impl<S, F, P> Sequential for MapItems<S, F, P>
where
    S: Sequential,
    F: Fn(S::Item) -> P,
{
    type Item = P;
    type Terminal = S::Terminal;

    fn into_next(self) -> Update<Self, Self::Item, Self::Terminal> {
        let MapItems { seq, f } = self;
        seq.into_next()
            .map_item(&f)
            .map_state(|next| MapItems::new(next, f))
    }
}
