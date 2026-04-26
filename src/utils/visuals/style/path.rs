use colored::Colorize;
use std::fmt::Display;

/// Apply path style for the item.
pub trait PathStyle: Display {
    /// Apply path style for the item.
    fn path_style(&self) -> String;
}

impl<T: Display> PathStyle for T {
    fn path_style(&self) -> String {
        format!("{}", self).underline().to_string()
    }
}
