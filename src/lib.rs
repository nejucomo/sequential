#![deny(unsafe_code, unused, missing_docs)]
//! A family of related traits exemplified by [Sequential]
pub mod sequential;

pub use self::sequential::{IntoSequential, Sequential};
