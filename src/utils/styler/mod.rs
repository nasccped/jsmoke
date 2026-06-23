//! Styling related content.
mod command_style;
mod output;

use command_style::CommandStyle;
pub use output::StylerOutput;
use std::ops::Deref;

/// Apply different kind of styles for any [`String`] compatible.
pub trait Styler: Deref<Target = str> {
    /// Gets the self item as a command style.
    fn command_style(&self) -> StylerOutput<String> {
        let result = CommandStyle::from(self.as_ref());
        StylerOutput::Item(result.to_string())
    }
}

impl<T: Deref<Target = str>> Styler for T {}
