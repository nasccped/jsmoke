//! Styling related content.
mod command_style;
mod output;

use colored::{ColoredString, Colorize};
use command_style::CommandStyle;
pub use output::StylerOutput;
use std::fmt::Display;

/// Apply different kind of styles for any [`String`] compatible.
pub trait Styler: Display {
    /// Gets the self item as a command style.
    fn command_style(&self) -> StylerOutput<String> {
        let temp = self.to_string();
        let result = CommandStyle::from(temp.as_str());
        StylerOutput::Item(result.to_string())
    }

    /// Gets the self item as strong style (bright white).
    fn strong_style(&self) -> StylerOutput<ColoredString> {
        StylerOutput::Item(self.to_string().bright_white().bold())
    }
}

impl<T: Display> Styler for T {}
