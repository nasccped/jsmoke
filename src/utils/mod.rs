//! # jsmoke utilities module
//!
//! A module that provides useful data structures along the runtime.
pub mod notifiers;
pub mod regex;
pub mod verbose;
pub mod visuals;

mod inner;
mod surely_unwrap;
mod trim_and_box;

pub use inner::Inner;
pub use surely_unwrap::SurelyUnwrap;
pub use trim_and_box::TrimAndBox;
