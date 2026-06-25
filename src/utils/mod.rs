//! # jsmoke utilities module
//!
//! A module that provides useful data structures along the runtime.
pub mod notifier;
mod reserveds;
mod strings;
pub mod styler;
mod verbose;

pub use reserveds::Reserveds;
pub use strings::StringUtils;
pub use verbose::Verbose;
