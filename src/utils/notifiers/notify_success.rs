use super::Tags;
use colored::Colorize;
use std::fmt::Display;

/// Add success notifier to the item. Works similar to [`super::NotifyFailure`] but for success
/// cases.
///
/// Items are required to also implement [`Display`].
pub trait NotifySuccess: Display {
    /// Returns the success title message as [`String`].
    fn get_success_message(&self) -> String {
        format!("{} {}", Tags::Done, self.to_string().bright_white())
    }
}
