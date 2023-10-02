use either::Either;

pub trait Sequential<I>: Sized {
    type Output;
    type Terminal;

    fn into_next_with(self, input: I) -> Either<(Self, Self::Output), Self::Terminal>;
}
