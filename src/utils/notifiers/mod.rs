//! # Notifier module
//!
//! Provides notify utilities like:
//! - `NotifySuccess` for successfully runtime.
//! - `NotifyFailure` for failed runtime.
//! - ...
mod notify_failure;
mod notify_success;
mod notify_warning;
mod tags;

use tags::Tags;

pub use notify_failure::NotifyFailure;
pub use notify_success::NotifySuccess;
pub use notify_warning::NotifyWarning;
