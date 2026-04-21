//! # jsmoke utilities module
//!
//! A module that provides useful data structures along the runtime.
pub mod notifiers;
pub mod regex;
pub mod verbose;
pub mod visuals;

mod inner;
mod optionally_from;
mod surely_unwrap;

pub use inner::{Inner, InnerAndConsume};
pub use optionally_from::OptionallyFrom;
pub use surely_unwrap::SurelyUnwrap;
