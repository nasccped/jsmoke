use std::fmt::Display;

/// Add failure notifying to the item. Useful for [`thiserror::Error`] and [`Display`]
/// implementors.
pub trait FailureNotifiable: Display {}
