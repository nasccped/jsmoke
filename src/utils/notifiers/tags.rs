use colored::Colorize;
use std::fmt::Display;

/// Private tag for title printing.
pub enum Tags {
    /// When it's ok.
    Done,
    /// When it's error.
    Fail,
    /// When it's warning.
    Warn,
}

const DONE_TAG_TITLE: &str = "done";
const FAIL_TAG_TITLE: &str = "fail";
const WARN_TAG_TITLE: &str = "warn";

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self {
                Self::Done => DONE_TAG_TITLE.bright_green(),
                Self::Fail => FAIL_TAG_TITLE.bright_red(),
                Self::Warn => WARN_TAG_TITLE.bright_yellow(),
            },
            ":".bright_white()
        )
    }
}
