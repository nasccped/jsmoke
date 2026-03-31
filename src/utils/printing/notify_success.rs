use super::NotifyTags;
use colored::Colorize;
use std::fmt::Display;

/// Notify to the user if an operation is successful.
///
/// Can be applied to a given type an print info based on it's inner data. To avoid hardcoded
/// implementing block, it autoimplements for types that implements [`Display`].
pub trait NotifySuccess {
    /// Notify the success to the user (prints to stdout).
    fn notify_success(&self);
}

// auto - implements to all types that implements display...
impl<T: Display> NotifySuccess for T {
    fn notify_success(&self) {
        println!("{} {}\n", NotifyTags::Ok, self.to_string().bright_white());
    }
}
