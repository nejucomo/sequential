#![deny(unsafe_code, unused, missing_docs)]
//! A [Sequential] trait for generating a sequence of values with an explicit termination value
mod andthen;
mod fnmut;
mod intosequential;
mod sequential;

pub use self::andthen::AndThen;
pub use self::fnmut::{from_fn_mut, SequentialFnMut};
pub use self::intosequential::IntoSequential;
pub use self::sequential::Sequential;

#[cfg(test)]
mod tests;
