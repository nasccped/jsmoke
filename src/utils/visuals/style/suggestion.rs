use super::quotes::QuoteIt;
use colored::Colorize;

/// A style trait for suggestion providing, like: "do `this` or do `that`".
pub trait SuggestionStyle {
    /// Turn the `self` item into a suggestion styled [`String`].
    fn suggestion_style(&self) -> String;
}

impl<T: ToString> SuggestionStyle for T {
    fn suggestion_style(&self) -> String {
        self.to_string().bright_yellow().simple_quote()
    }
}
