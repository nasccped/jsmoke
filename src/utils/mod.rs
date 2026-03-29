//! # jsmoke utilities module
//!
//! A module that provides useful data structures along the runtime.
mod optionally_from;
pub mod printer;
mod surely_unwrap;
pub mod verbose;
pub mod visuals;

pub use optionally_from::OptionallyFrom;
pub use surely_unwrap::SurelyUnwrap;
