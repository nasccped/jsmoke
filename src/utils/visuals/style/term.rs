use colored::{ColoredString, Colorize};

/// Apply term styling to the item. Requires item to implements [`Colorize`] and [`Clone`] trait.
pub trait TermStyle: Colorize + Clone {
    /// Apply styles form term like "there's some `constraints` over it..."
    fn term_style(&self) -> ColoredString;
}

impl<T: Colorize + Clone> TermStyle for T {
    fn term_style(&self) -> ColoredString {
        self.clone().italic()
    }
}
