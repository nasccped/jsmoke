#![allow(dead_code)]
use super::{FailureNotifiable, SuccessNotifiable, WarningNotifiable, tags::Tags};
use colored::Colorize;
use std::fmt::Display;

/// Struct designated for output notify (display).
///
/// Usefull for indiscriminated [`println`] calls.
pub struct OutputNotifier {}

impl OutputNotifier {
    /// Notify success items ([`SuccessNotifiable`]).
    fn notify_success(item: impl SuccessNotifiable) {
        println!("{} {}", Tags::Done, item.to_string().bright_white());
    }

    /// Notify failure items ([`FailureNotifiable`]).
    fn notify_failure(item: impl FailureNotifiable) {
        eprintln!("{} {}", Tags::Fail, item.to_string().bright_white());
    }

    /// Notify failure items ([`FailureNotifiable`]).
    fn notify_warning(item: impl WarningNotifiable, is_err: bool) {
        let message = format!("{} {}", Tags::Warn, item.to_string().bright_white());
        if is_err {
            eprintln!("{}", message);
        } else {
            println!("{}", message);
        }
    }

    /// Notify non taged items (not implementors of `*Notifiable`).
    fn notify_non_taged(item: impl Display, is_err: bool) {
        if is_err {
            eprintln!("{}", item);
        } else {
            println!("{}", item);
        }
    }
}
