//! # jsmoke utilities module
//!
//! A module that provides useful data structures along the runtime.
pub mod notifiers;
pub mod regex;
pub mod verbose;
pub mod visuals;

mod inner;
mod surely_unwrap;

pub use inner::Inner;
pub use surely_unwrap::SurelyUnwrap;
