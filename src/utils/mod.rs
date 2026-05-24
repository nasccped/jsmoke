//! # jsmoke utilities module
//!
//! A module that provides useful data structures along the runtime.
pub mod notifiers;
// pub mod verbose;
// pub mod visuals;

mod input_fix;
mod may_from;
mod surely_unwrap;

pub use input_fix::InputFix;
pub use may_from::MayFrom;
pub use surely_unwrap::SurelyUnwrap;
