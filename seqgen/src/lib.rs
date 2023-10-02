#![deny(unsafe_code, unused, missing_docs)]
//! The [SeqGen] trait and supporting types for abstract sequential generation of outputs and explicit termination
mod intosg;
mod seqgen;

pub use self::intosg::IntoSeqGen;
pub use self::seqgen::SeqGen;
