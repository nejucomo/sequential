use crate::Sequential;

/// Types which can be converted into a [Sequential] with specific item and termination types
///
/// A blanket implementation ensures all [Sequential] types provide [IntoSequential], analogous to [Iterator] and [IntoIterator].
pub trait IntoSequential {
    /// The [Item](Sequential::Item) of [Into](IntoSequential::Into)
    type Item;
    /// The [Terminal](Sequential::Terminal) of [Into](IntoSequential::Into)
    type Terminal;
    /// The [Sequential] type `self` converts into
    type Into: Sequential<Item = Self::Item, Terminal = Self::Terminal>;

    /// Convert `self` into a [Sequential] type
    fn into_sequential(self) -> Self::Into;
}

impl<S> IntoSequential for S
where
    S: Sequential,
{
    type Item = <S as Sequential>::Item;
    type Terminal = <S as Sequential>::Terminal;
    type Into = S;

    fn into_sequential(self) -> Self::Into {
        self
    }
}
