#![deny(unsafe_code, unused, missing_docs)]
//! A [Sequential] trait for generating a sequence of values with an explicit termination value
mod andthen;
mod fnmut;
mod intosequential;
mod sequential;
mod termonerror;

pub use self::andthen::AndThen;
pub use self::fnmut::{from_fn_mut, SequentialFnMut};
pub use self::intosequential::IntoSequential;
pub use self::sequential::Sequential;
pub use self::termonerror::{terminate_on_error, TerminateOnError};

#[cfg(test)]
mod tests;
