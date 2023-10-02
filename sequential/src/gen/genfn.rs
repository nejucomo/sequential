use crate::Generator;
use either::Either;

#[derive(derive_more::From)]
pub struct GeneratorFn<F, O, T>(F)
where
    F: FnMut() -> Either<O, T>;

impl<F, O, T> Generator for GeneratorFn<F, O, T>
where
    F: FnMut() -> Either<O, T>,
{
    type Output = O;
    type Terminal = T;

    fn into_next(mut self) -> Either<(Self, O), T> {
        let r = self.0();
        r.map_left(|x| (self, x))
    }
}
