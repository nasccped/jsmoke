use colored::Colorize;
use std::fmt::Display;

/// Apply numbering style for the item.
pub trait NumberStyle: Display {
    /// Apply numbering style for the item.
    fn number_style(&self) -> String;
}

impl<T: Display> NumberStyle for T {
    fn number_style(&self) -> String {
        format!("{self}").bright_green().to_string()
    }
}
