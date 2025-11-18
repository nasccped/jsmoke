//! # Color Highlights
//!
//! Provide fast-hand colored printing using [`colored`] crate and
//! private functions to avoid code repetition.

use crate::strings::{auto_trim::AutoTrim, chunk_by_any::ChunkByAny};
use colored::Colorize;

/// Tick char literal (avoid magic values).
const TICK_CHAR: char = '`';

/// Module separator literal (avoid magic values).
const MODULE_SEPARATOR: &str = "::";

/// Type elements commonly present within type syntax.
const TYPE_ELEMENTS: [&str; 6] = ["&", "'a", "<>", "<", ">", "_"];

/// Turns a [`str`] into bright green (private).
fn b_green(value: &str) -> String {
    value.bright_green().to_string()
}

/// Turns a [`str`] into bright blue (private).
fn b_blue(value: &str) -> String {
    value.bright_blue().to_string()
}

/// Turns a [`str`] into bright magenta (private).
fn b_magenta(value: &str) -> String {
    value.bright_magenta().to_string()
}

/// Turns a [`str`] into bright cyan (private).
fn b_cyan(value: &str) -> String {
    value.bright_cyan().to_string()
}

/// Turns a [`str`] into bright white (private).
fn b_white(value: &str) -> String {
    value.bright_white().to_string()
}

/// Apply color highlights based on the item kind.
///
/// This trait allows to avoid different coloring through similar
/// kind of items (+ easy extension and fixing).
pub trait ColorHighlights<'a> {
    /// Apply module highlight to an item (like [`std`],
    /// [`crate::types`] and so on).
    fn module_highlight(&'a self) -> String;

    /// Apply type highlight to an item (like [`i32`] or [`f64`]).
    fn type_highlight(&'a self) -> String;

    /// Apply machine's core commands (`mkdir`, `echo`, ...) color
    /// highlight.
    fn command_highlight(&'a self) -> String;

    /// Apply string's highlight (like `"this"` and `"that"`).
    fn string_highlight(&'a self) -> String;
}

// impl for any type that implements `AutoTrim` (auto implements
// this trait for `str`, `String` references).
impl<'a, T: AutoTrim<'a>> ColorHighlights<'a> for T {
    fn module_highlight(&'a self) -> String {
        let (ticks, item) = ticking::untick(self.auto_trim());
        let mod_str = item
            .split(MODULE_SEPARATOR)
            .map(b_magenta)
            .collect::<Vec<_>>()
            .join(b_white(MODULE_SEPARATOR).as_str());
        if ticks {
            ticking::retick(mod_str)
        } else {
            mod_str
        }
    }

    fn type_highlight(&'a self) -> String {
        let (ticks, item) = ticking::untick(self.auto_trim());
        let items = item
            .chunk_by_any(TYPE_ELEMENTS)
            .iter()
            .fold(String::new(), |mut s, item| {
                match item.auto_trim() {
                    // `any` and `lifetime` items are special.
                    "_" => s.push_str(b_magenta("_").as_str()),
                    "'a" => s.push_str((b_white("'") + b_cyan("a").as_str()).as_str()),

                    // otherwise
                    x if TYPE_ELEMENTS.contains(&x) => s.push_str(b_white(x).as_str()),
                    x => s.push_str(b_green(x).as_str()),
                }
                s
            });
        if ticks { ticking::retick(items) } else { items }
    }

    fn command_highlight(&'a self) -> String {
        let (ticks, item) = ticking::untick(self.auto_trim());
        let item = b_blue(item);
        if ticks { ticking::retick(item) } else { item }
    }

    fn string_highlight(&'a self) -> String {
        let (ticks, item) = ticking::untick(self.auto_trim());
        let item = b_cyan(item);
        if ticks { ticking::retick(item) } else { item }
    }
}

mod ticking {
    //! Ticking module. Remove and apply code ticks. Tiny and private
    //! since it's only used by the [`super`] module.

    use super::*;

    /// "Untick" the provided [`str`] slice and return a tuple containing
    /// `(bool, &str)` where [`bool`] means if the string is 'ticked'
    /// and &[`str`] is the 'unticked' value (return `self` if no ticks).
    pub fn untick(value: &str) -> (bool, &str) {
        if let Some(item_str) = value
            .strip_prefix(TICK_CHAR)
            .and_then(|some| some.strip_suffix(TICK_CHAR))
        {
            (true, item_str)
        } else {
            (false, value)
        }
    }

    /// Reapply ticks (already colored) around an item.
    pub fn retick(inner: impl std::fmt::Display) -> String {
        let tick = b_white("`");
        format!("{}{}{}", tick, inner, tick)
    }
}
