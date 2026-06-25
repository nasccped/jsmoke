//! # Notifier module
//!
//! Provides notify utilities like:
//! - `NotifySuccess` for successfully runtime.
//! - `NotifyFailure` for failed runtime.
//! - ...
mod _private;

use super::Verbose;
use crate::{cli::AppParseFail, services::error::ServiceParseError};
use _private::{IntoBrightWhite, Tags};
use std::{borrow::Cow, fmt::Display};

/// Struct designated for output notify (display).
///
/// Helps to avoid indiscriminated [`println`] calls.
#[derive(Clone, Copy)]
pub struct Notifier {
    /// If content is being printed to `stderr`.
    is_err: bool,
}

pub trait Notifiable {
    fn get_notify_message(&self) -> Cow<'_, str>;
}

impl<T: Display> Notifiable for T {
    fn get_notify_message(&self) -> Cow<'_, str> {
        Cow::Owned(self.to_string())
    }
}

impl From<&AppParseFail> for Notifier {
    fn from(value: &AppParseFail) -> Self {
        // set err to true since parse error is always an error
        Self {
            is_err: value.is_err(),
        }
    }
}

impl From<&ServiceParseError<'_>> for Notifier {
    fn from(_: &ServiceParseError<'_>) -> Self {
        Self { is_err: true }
    }
}

impl Notifier {
    /// Toggle on the `is_err` inner value (file stream pointing to `stderr`) + returns the
    /// reference to itself.
    ///
    /// This action isn't reversible:
    /// > Since it's an error, it'll always be an error...
    pub fn toggle_error(&mut self) -> &Self {
        self.is_err = true;
        self
    }

    /// Notify success cases to `stdout` (even when `is_err` field is set to true - consider using
    /// [`Notifier::notify_failure`] or [`Notifier::notify_simple`] if necessary).
    pub fn notify_success(&self, item: &dyn Notifiable) {
        println!("{} {}", Tags::Done, item.into_bright_white());
    }

    /// Set the `is_err` field to true and then, print the content to `stderr`.
    ///
    /// *Note:* Since it's an error, all other calls ([`Notifier::notify_success`] except) will
    /// print to `stderr` too.
    pub fn notify_failure(&mut self, item: &dyn Notifiable) {
        self.toggle_error();
        eprintln!("{} {}", Tags::Fail, item.into_bright_white());
    }

    /// Notify warning-like info at a file stream. Decides `stdout`/`stderr` based on `is_err`
    /// inner field.
    ///
    /// Consider changing it with the [`Notifier::toggle_error`] function.
    pub fn notify_warning(&self, item: &dyn Notifiable) {
        let message = format!("{} {}", Tags::Warn, item.into_bright_white());
        local_print(self, message);
    }

    /// Notify simple stuff (since they impl [`Display`]). Decides `stdout`/`stderr` based on
    /// `is_err` inner field.
    ///
    /// Consider changing it with the [`Notifier::toggle_error`] function.
    pub fn notify_simple(&self, item: impl Display) {
        local_print(self, item);
    }

    /// Notify the verbose message to `stdout`/`stderr` (based on `is_err` inner field).
    ///
    /// Note that this function can also do nothing since [`Verbose::get_verbose_message`] returns
    /// an [`Option`] over [`Cow`], where [`None`] means `no message`. Make sure to return a
    /// message when implementing it to some type.
    pub fn notify_verbose(&self, item: &dyn Verbose) {
        if let Some(message) = item.get_verbose_message() {
            local_print(self, format!("\n{}\n", message));
        }
    }
}

/// Shorthand function to decides where to print (`stdout`/`stderr`). Better than using if-else on
/// three different functions.
///
/// This function is also `#inline` to avoid instruction jump overhead.
#[inline]
fn local_print(notifier: &Notifier, item: impl Display) {
    if notifier.is_err {
        eprintln!("{}", item);
    } else {
        println!("{}", item);
    }
}
