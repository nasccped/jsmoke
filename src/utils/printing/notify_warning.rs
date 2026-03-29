use super::NotifyTags;
use colored::Colorize;
use std::fmt::Display;

/// Notify to the user if an operation has generated a warning.
///
/// Can be applied to a given type an print info based on it's inner data. To avoid hardcoded
/// implementing block, it's require that the type implements [`Display`].
pub trait NotifyWarning: Display {
    /// Notify the warning to the user (prints to stdout).
    fn notify_warning(&self) {
        println!(
            "{} {}",
            NotifyTags::Warning,
            self.to_string().bright_white()
        );
    }
}
