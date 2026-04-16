use colored::Colorize;
use std::fmt::Display;

/// Apply dollar sign to the item. Works well with [`super::CommandStyle`].
pub trait DollarIt: Display {
    /// Apply dollar on it.
    fn dollar_it(&self) -> String;
}

impl<T: Display> DollarIt for T {
    fn dollar_it(&self) -> String {
        format!("{} {}", "$".bright_white(), self)
    }
}
