#![deny(unsafe_code, unused, missing_docs)]
//! The [Sequential] trait and supporting types for abstract sequential processing over inputs, outputs, and explicit termination
mod intoseq;
mod sequential;

pub use self::intoseq::IntoSequential;
pub use self::sequential::Sequential;
