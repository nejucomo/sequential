#![deny(unused, missing_docs)]
#![forbid(unsafe_code)]
#![feature(try_trait_v2)]
#![doc = include_str!("../README.md")]

mod andthen;
mod fnmut;
mod intosequential;
mod mapitems;
mod mapterminal;
mod sequential;
mod termonerr;
mod transformnext;

pub use self::andthen::AndThen;
pub use self::fnmut::{from_fn_mut, SequentialFnMut};
pub use self::intosequential::IntoSequential;
pub use self::mapitems::MapItems;
pub use self::mapterminal::MapTerminal;
pub use self::sequential::Sequential;
pub use self::termonerr::TerminateOnErr;
pub use self::transformnext::TransformNext;

#[cfg(test)]
mod tests;
