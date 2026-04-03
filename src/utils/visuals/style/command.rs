use super::ticks::TickIt;
use colored::Colorize;

/// Apply color style like as command (requires item to implements [`ToString`] trait).
pub trait CommandStyle: ToString {
    /// Apply a color style to the designed item. The color being applied depends on how the impl
    /// was done.
    fn command_style(&self) -> String;
}

impl<T: ToString> CommandStyle for T {
    fn command_style(&self) -> String {
        self.to_string().bright_green().tick_it()
    }
}
