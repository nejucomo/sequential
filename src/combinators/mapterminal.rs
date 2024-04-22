use crate::{Sequential, Update};

/// Map the [Terminal](Sequential::Terminal) of a [Sequential]
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
    type Item = S::Item;
    type Terminal = U;

    fn into_next(self) -> Update<Self, Self::Item, Self::Terminal> {
        let MapTerminal { seq, f } = self;
        seq.into_next()
            .map_terminal(&f)
            .map_state(|next| MapTerminal::new(next, f))
    }
}
