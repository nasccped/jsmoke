//! # jsmoke utilities module
//!
//! A module that provides useful data structures along the runtime.
mod optionally_from;
pub mod printing;
mod surely_unwrap;
mod verbose;
pub mod visuals;

pub use optionally_from::OptionallyFrom;
pub use surely_unwrap::SurelyUnwrap;
pub use verbose::Verbose;
