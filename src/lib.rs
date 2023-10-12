#![deny(unsafe_code, unused, missing_docs)]
//! A [Sequential] trait for generating a sequence of values with an explicit termination value
//!
//! The fundamental method is [Sequential::into_next]:
//!
//! ```rust,ignore
//! fn into_next(self) -> Either<(Self, Self::Output), Self::Terminal>;
//! ```
//!
//! This either produces a means of contuing (via `Self`) and an [Output](Sequential::Output), or else a [Terminal](Sequential::Terminal) value. Because this method consumes `self`, it ensures the [Sequential] state is dropped upon termination.
//!
//! # Example
//!
//! ```
//! use sequential::Sequential;
//! use std::io::{BufRead, BufReader, Read};
//!
//! fn count_lines_and_chars<R>(r: R) -> std::io::Result<(usize, usize)>
//! where
//!     R: Read,
//! {
//!     let mut lines = 0;
//!     let mut chars = 0;
//!     let seq = sequential::terminate_on_error(BufReader::new(r).lines());
//!     seq.for_each(|line| {
//!         lines += 1;
//!         chars += line.chars().count();
//!     })?;
//!
//!     Ok((lines, chars))
//! }
//! ```

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
