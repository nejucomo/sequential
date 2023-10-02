pub mod gen;
pub mod seq;

pub use self::gen::{Generator, GeneratorFn};
pub use self::seq::{IntoSequential, Sequential};
