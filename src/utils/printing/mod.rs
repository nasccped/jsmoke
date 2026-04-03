//! # Printing module
//!
//! Provides printing utilities like:
//! - `NotifySuccess` for successfully runtime.
//! - `NotifyFailure` for failed runtime.
//! - `NotifyWarning` for non-fatal status.
//! - `SimpleNotify` for just simple printing.
mod notify_failure;
mod notify_success;
mod notify_warning;
mod simple_notify;

use colored::Colorize;
pub use notify_failure::NotifyFailure;
pub use notify_success::NotifySuccess;
pub use notify_warning::NotifyWarning;
pub use simple_notify::SimpleNotify;
use std::fmt::Display;

/// Private tag for title printing.
enum NotifyTags {
    /// When it's ok.
    Ok,
    /// When it's error.
    Error,
    /// When it's warning.
    Warning,
}

impl Display for NotifyTags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self {
                Self::Ok => "ok".bright_green(),
                Self::Error => "fail".bright_red(),
                Self::Warning => "warn".bright_yellow(),
            },
            ":".bright_white()
        )
    }
}
