//! Styling related content.
mod command_style;
mod list_style;
mod output;

use colored::{ColoredString, Colorize};
use command_style::CommandStyle;
pub use list_style::ListStyle;
pub use output::StylerOutput;
use std::ops::Deref;
use std::{fmt::Display, sync::LazyLock};

/// Tick item for reusable formatting.
static TICK: LazyLock<ColoredString> = LazyLock::new(|| "`".bright_white());

/// Quote item for reusable formatting.
static QUOTE: LazyLock<ColoredString> = LazyLock::new(|| "'".bright_white());

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

    /// Gets the self item as number style (bright green).
    fn number_style(&self) -> StylerOutput<ColoredString> {
        StylerOutput::Item(self.to_string().bright_green())
    }

    /// Gets the self item as suggestion style (cyan with white quotes).
    fn suggestion_style(&self) -> StylerOutput<String> {
        let q = QUOTE.deref();
        StylerOutput::Item(format!("{}{}{}", q, self.to_string().cyan(), q))
    }

    /// Gets the self item as term style (italic).
    fn term_style(&self) -> StylerOutput<ColoredString> {
        StylerOutput::Item(self.to_string().italic())
    }

    /// Gets the self item as a note block (with tag).
    fn note_style(&self) -> StylerOutput<String> {
        StylerOutput::Item(format!("{} {}", "Note:".bright_cyan(), self))
    }

    /// A style reserved only for special cases (bright yellow).
    fn special_style(&self) -> StylerOutput<ColoredString> {
        StylerOutput::Item(self.to_string().bright_yellow())
    }
}

impl<T: Display> Styler for T {}
