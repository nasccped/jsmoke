use colored::Colorize;
use std::fmt::Display;

const ITEM_INDICATOR: &str = ">";

/// Put item list styling to the `self` item, like as:
///
/// > Item 1
/// > Item 2
/// > ...
pub trait ItemList: Display {
    /// Put item style to the self item. Note that it prints a non ordered item (such as
    /// `* example` instead of `1. example`).
    fn item_list_style(&self) -> String;
}

impl<T: Display> ItemList for T {
    fn item_list_style(&self) -> String {
        format!("{} {}", ITEM_INDICATOR.bright_cyan(), self)
    }
}
