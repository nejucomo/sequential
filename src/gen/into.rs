use crate::Generator;

pub trait IntoGenerator {
    type Output;
    type Terminal;
    type Gen: Generator;

    fn into_generator(self) -> Self::Gen;
}

impl<G> IntoGenerator for G
where
    G: Generator,
{
    type Output = <G as Generator>::Output;
    type Terminal = <G as Generator>::Terminal;
    type Gen = G;

    fn into_generator(self) -> Self::Gen {
        self
    }
}
