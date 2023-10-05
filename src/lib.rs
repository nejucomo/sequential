#![deny(unsafe_code, unused, missing_docs)]
//! A family of related traits exemplified by [Sequential]
pub mod emitter;
pub mod sequential;

pub use self::emitter::{Emitter, IntoEmitter};
pub use self::sequential::{IntoSequential, Sequential};
