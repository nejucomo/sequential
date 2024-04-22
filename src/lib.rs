#![deny(unused, missing_docs)]
#![forbid(unsafe_code)]
#![feature(try_trait_v2)]
#![doc = include_str!("../README.md")]

pub mod combinators;
mod fnmut;
mod intosequential;
mod sequential;
mod transformnext;

pub use self::fnmut::{from_fn_mut, SequentialFnMut};
pub use self::intosequential::IntoSequential;
pub use self::sequential::Sequential;
pub use self::transformnext::TransformNext;

#[cfg(test)]
mod tests;
