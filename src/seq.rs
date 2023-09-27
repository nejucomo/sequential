mod into;

use crate::Generator;
use either::Either;

pub use self::into::IntoSequential;

pub trait Sequential<I>: Sized {
    type Output;
    type Terminal;

    fn into_next_with(self, input: I) -> Either<(Self, Self::Output), Self::Terminal>;
}

impl<G> Sequential<()> for G
where
    G: Generator,
{
    type Output = <G as Generator>::Output;
    type Terminal = <G as Generator>::Terminal;

    fn into_next_with(self, _: ()) -> Either<(Self, Self::Output), Self::Terminal> {
        self.into_next()
    }
}
