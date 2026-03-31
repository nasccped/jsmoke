use super::NotifyTags;
use colored::Colorize;
use std::fmt::Display;

/// Notify to the user if an error occurs.
///
/// Can be applied to a given type an print info based on it's inner data. Works well with
/// [`thiserror::Error`] macro.
pub trait NotifyError {
    /// Notify the error to the user (prints to stderr).
    fn notify_error(&self);
}

// auto - implements to all types that implements display...
impl<T: Display> NotifyError for T {
    fn notify_error(&self) {
        eprintln!(
            "{} {}\n",
            NotifyTags::Error,
            self.to_string().bright_white()
        );
    }
}
