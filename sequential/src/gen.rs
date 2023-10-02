mod genfn;
mod into;
mod intoiter;

use either::Either::{self, *};

pub use self::genfn::GeneratorFn;
pub use self::into::IntoGenerator;
pub use self::intoiter::{into_iter, IntoIter};

pub trait Generator: Sized {
    type Output;
    type Terminal;

    fn into_next(self) -> Either<(Self, Self::Output), Self::Terminal>;
}

impl<I> Generator for I
where
    I: Iterator,
{
    type Output = <I as Iterator>::Item;
    type Terminal = ();

    fn into_next(mut self) -> Either<(Self, Self::Output), Self::Terminal> {
        if let Some(output) = self.next() {
            Left((self, output))
        } else {
            Right(())
        }
    }
}
