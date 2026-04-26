use colored::ColoredString;
use std::fmt::Display;

const SIMPLE_QUOTE: &str = "'";
const DOUBLE_QUOTE: &str = "\"";

/// Function alias for str slice coloring.
type ColorizeFunction = for<'a> fn(&'a str) -> ColoredString;

/// Apply quotes to the item. Requires item to implements [`Display`] trait.
pub trait QuoteIt: Display {
    /// Apply simple quotes on it.
    ///
    /// `Self` type requires to implements [`Display`], and the function parameter is the color
    /// being applied.
    fn simple_quote(&self, func: ColorizeFunction) -> String;

    /// Apply double quotes on it.
    ///
    /// `Self` type requires to implements [`Display`], and the function parameter is the color
    /// being applied.
    #[allow(dead_code)]
    fn double_quote(&self, func: ColorizeFunction) -> String;
}

impl<T: Display> QuoteIt for T {
    fn simple_quote(&self, func: fn(&str) -> ColoredString) -> String {
        apply_quote(self, func, SIMPLE_QUOTE)
    }

    fn double_quote(&self, func: fn(&str) -> ColoredString) -> String {
        apply_quote(self, func, DOUBLE_QUOTE)
    }
}

/// Apply quote privately, since [`QuoteIt::simple_quote`] and [`QuoteIt::double_quote`] does
/// almost the same.
fn apply_quote<T: Display>(item: T, color: ColorizeFunction, quote: &str) -> String {
    let quote = color(quote);
    format!("{q}{item}{q}", q = quote)
}
