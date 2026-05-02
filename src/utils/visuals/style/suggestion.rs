use super::quotes::QuoteIt;
use colored::Colorize;

/// A style trait for suggestion providing, like: "do `this` or do `that`".
pub trait SuggestionStyle: ToString {
    /// Turn the `self` item into a suggestion styled [`String`].
    fn suggestion_style(&self) -> String;
}

impl<T: ToString> SuggestionStyle for T {
    fn suggestion_style(&self) -> String {
        let item = self.to_string().cyan();
        let f = |x: &'_ str| Colorize::cyan(x);
        item.simple_quote(f).italic().to_string()
    }
}
