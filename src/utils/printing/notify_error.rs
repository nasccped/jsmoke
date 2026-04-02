use super::NotifyTags;
use colored::Colorize;
use std::fmt::Display;

/// Notify to the user if an error occurs.
///
/// Can be applied to a given type an print info based on it's inner data. Works well with
/// [`thiserror::Error`] macro.
pub trait NotifyFailure {
    /// Notify the error to the user (prints to stderr).
    fn notify_failure(&self);
}

// auto - implements to all types that implements display...
impl<T: Display> NotifyFailure for T {
    fn notify_failure(&self) {
        eprintln!(
            "{} {}\n",
            NotifyTags::Error,
            self.to_string().bright_white()
        );
    }
}
