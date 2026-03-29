//! # Printing module
//!
//! Provides printing utilities like:
//! - `NotifySuccess` for successfully runtime.
//! - `NotifyError` for failed runtime.
//! - `NotifyWarning` for non-fatal status.
//! - `SimpleNotify` for just simple printing.
mod notify_error;
mod notify_success;
mod notify_warning;
mod simple_notify;

use colored::Colorize;
pub use notify_error::NotifyError;
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
                Self::Error => "error".bright_red(),
                Self::Warning => "warning".bright_yellow(),
            },
            ":".bright_white()
        )
    }
}
