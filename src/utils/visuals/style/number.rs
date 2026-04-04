use colored::{ColoredString, Colorize};
use std::fmt::Display;

/// Apply numbering style for the item.
pub trait NumberStyle: Display {
    /// Apply numbering style for the item.
    fn number_style(&self) -> ColoredString;
}

impl<T: Display> NumberStyle for T {
    fn number_style(&self) -> ColoredString {
        format!("{}", self).bright_green()
    }
}
