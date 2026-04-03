use colored::Colorize;
use std::fmt::Display;

/// Apply ticks to the item. Requires item to implements [`Display`] trait.
pub trait TickIt: Display {
    /// Apply ticks on it.
    fn tick_it(&self) -> String;
}

impl<T: Display> TickIt for T {
    fn tick_it(&self) -> String {
        let t = "`".bright_white();
        format!("{}{}{}", t, self, t)
    }
}
