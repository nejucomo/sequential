use crate::Generator;

#[derive(Copy, Clone, Debug)]
pub struct IntoIter<G>(Option<G>)
where
    G: Generator<Terminal = ()>;

pub fn into_iter<G>(g: G) -> IntoIter<G>
where
    G: Generator<Terminal = ()>,
{
    IntoIter(Some(g))
}

impl<G> Iterator for IntoIter<G>
where
    G: Generator<Terminal = ()>,
{
    type Item = <G as Generator>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(g) = self.0.take() {
            if let Some((newg, x)) = g.into_next().left() {
                self.0 = Some(newg);
                Some(x)
            } else {
                None
            }
        } else {
            None
        }
    }
}
