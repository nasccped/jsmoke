use super::Tags;
use colored::Colorize;
use std::fmt::Display;

/// Add failure notifying to the item. Useful for [`thiserror::Error`] and [`Display`]
/// implementors.
pub trait NotifyFailure: Display {
    /// Returns the fail title message as [`String`].
    fn get_fail_message(&self) -> String {
        format!("{} {}", Tags::Fail, self.to_string().bright_white())
    }
}
