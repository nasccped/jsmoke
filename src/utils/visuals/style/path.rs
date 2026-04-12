use colored::{ColoredString, Colorize};
use std::fmt::Display;

/// Apply path style for the item.
pub trait PathStyle: Display {
    /// Apply path style for the item.
    fn path_style(&self) -> ColoredString;
}

impl<T: Display> PathStyle for T {
    fn path_style(&self) -> ColoredString {
        format!("{}", self).underline()
    }
}
