use colored::Colorize;
use std::fmt::Display;

/// Apply quotes to the item. Requires item to implements [`Display`] trait.
pub trait QuoteIt: Display {
    /// Apply simple quotes on it.
    fn simple_quote(&self) -> String;
    /// Apply double quotes on it.
    fn double_quote(&self) -> String;
}

impl<T: Display> QuoteIt for T {
    fn simple_quote(&self) -> String {
        let q = "'".bright_white();
        format!("{}{}{}", q, self, q)
    }

    fn double_quote(&self) -> String {
        let q = "\"".bright_white();
        format!("{}{}{}", q, self, q)
    }
}
