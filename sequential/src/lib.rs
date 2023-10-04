#![deny(unsafe_code, unused, missing_docs)]
//! The [Sequential] trait and supporting types for abstract sequential processing over inputs, outputs, and explicit termination
//!
//! # Related Crates
//! - `seqgen` - provides the `SeqGen` trait which is analogous to [Sequential] except with no inputs.
//! - `sequential-async` - provies an asynchronous equivalent to this crate.
mod intoseq;
mod sequential;

pub use self::intoseq::IntoSequential;
pub use self::sequential::Sequential;
