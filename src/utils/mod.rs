//! # jsmoke utilities module
//!
//! A module that provides useful data structures along the runtime.
mod input_fix;
mod may_from;
mod strings;
mod surely_unwrap;
mod verbose;

pub use input_fix::InputFix;
pub use may_from::MayFrom;
pub use strings::StringUtils;
pub use surely_unwrap::SurelyUnwrap;
pub use verbose::Verbose;
