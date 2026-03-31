use super::NotifyTags;
use colored::Colorize;
use std::fmt::Display;

/// Notify to the user if an operation has generated a warning.
///
/// Can be applied to a given type an print info based on it's inner data. To avoid hardcoded
/// implementing block, it's auto-implemented for all types that implements [`Display`].
pub trait NotifyWarning {
    /// Notify the warning to the user (prints to stdout).
    fn notify_warning(&self);
}

impl<T: Display> NotifyWarning for T {
    fn notify_warning(&self) {
        println!(
            "{} {}\n",
            NotifyTags::Warning,
            self.to_string().bright_white()
        );
    }
}
