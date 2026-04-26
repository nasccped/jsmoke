use colored::Colorize;

/// Apply term styling to the item. Requires item to implements [`Colorize`] and [`Clone`] trait.
pub trait TermStyle {
    /// Apply styles form term like "there's some `constraints` over it..."
    fn term_style(&self) -> String;
}

impl TermStyle for String {
    fn term_style(&self) -> String {
        self.italic().to_string()
    }
}

impl TermStyle for &str {
    fn term_style(&self) -> String {
        self.italic().to_string()
    }
}
