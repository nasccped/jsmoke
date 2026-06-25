use super::Notifiable;
use colored::{ColoredString, Colorize};
use std::fmt::Display;

/// Just turn the `self` object into a bright white [`ColoredString`] (better than calling
/// `item.get_notify_message().bright_way()` all the way).
#[allow(clippy::wrong_self_convention)]
pub trait IntoBrightWhite {
    /// Converts the self item into a bright white [`ColoredString`].
    fn into_bright_white(&self) -> ColoredString;
}

impl<T: Notifiable + ?Sized> IntoBrightWhite for T {
    fn into_bright_white(&self) -> ColoredString {
        self.get_notify_message().bright_white()
    }
}

/// Private tag for title printing.
pub enum Tags {
    /// When it's ok.
    Done,
    /// When it's error.
    Fail,
    /// When it's warning.
    Warn,
}

/// For done tasks.
const DONE_TAG_TITLE: &str = "done";

/// For fail tasks.
const FAIL_TAG_TITLE: &str = "fail";

/// For warn tasks.
const WARN_TAG_TITLE: &str = "warn";

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag = match self {
            Self::Done => DONE_TAG_TITLE.bright_green(),
            Self::Fail => FAIL_TAG_TITLE.bright_red(),
            Self::Warn => WARN_TAG_TITLE.bright_yellow(),
        };
        write!(f, "{}{}", tag, ":".bright_white())
    }
}
