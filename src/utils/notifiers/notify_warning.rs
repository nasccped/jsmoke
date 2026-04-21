use super::Tags;
use colored::Colorize;
use std::fmt::Display;

/// Add warning notifier to the item. Works similar to [`super::NotifyFailure`] but for warning
/// cases.
///
/// Items are required to also implement [`Display`].
pub trait NotifyWarning: Display {
    /// Returns the warning title message as [`String`].
    fn get_warning_message(&self) -> String {
        format!("{} {}", Tags::Warn, self.to_string().bright_white())
    }
}
