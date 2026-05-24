//! # Notifier module
//!
//! Provides notify utilities like:
//! - `NotifySuccess` for successfully runtime.
//! - `NotifyFailure` for failed runtime.
//! - ...
mod failure_notifiable;
mod output_notifier;
mod success_notifiable;
mod tags;
mod warning_notifiable;

pub use failure_notifiable::FailureNotifiable;
pub use output_notifier::OutputNotifier;
pub use success_notifiable::SuccessNotifiable;
pub use warning_notifiable::WarningNotifiable;
