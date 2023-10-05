use crate::Emitter;

/// Types which can be converted into a [Emitter] process with specific output and termination types
///
/// A blanked implementation ensures all [Emitter] types provide [IntoEmitter], analogous to [Iterator] and [IntoIterator].
pub trait IntoEmitter {
    /// The output of [IntoEmitter::Into]
    type Output;
    /// The terminal of [IntoEmitter::Into]
    type Terminal;
    /// The [Emitter] value `self` converts into
    type Into: Emitter<Output = Self::Output, Terminal = Self::Terminal>;

    /// Convert `self` into a [Emitter] type
    fn into_emitter(self) -> Self::Into;
}

impl<E> IntoEmitter for E
where
    E: Emitter,
{
    type Output = <E as Emitter>::Output;
    type Terminal = <E as Emitter>::Terminal;
    type Into = E;

    fn into_emitter(self) -> Self::Into {
        self
    }
}
