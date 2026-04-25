use colored::{ColoredString, Colorize};

/// Apply term styling to the item. Requires item to implements [`Colorize`] and [`Clone`] trait.
pub trait TermStyle {
    /// Apply styles form term like "there's some `constraints` over it..."
    fn term_style(&self) -> ColoredString;
}

impl TermStyle for String {
    fn term_style(&self) -> ColoredString {
        self.italic()
    }
}

impl TermStyle for &str {
    fn term_style(&self) -> ColoredString {
        self.italic()
    }
}
