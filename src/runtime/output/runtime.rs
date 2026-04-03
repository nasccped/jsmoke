use super::{FailureConstraint, SuccessConstraint};
use crate::utils::Verbose;
use std::process::ExitCode;

/// Type alias for jsmoke result output.
///
/// It holds the [`Result<Box<dyn DoneConstraint>, Box<dyn FailedConstraint>>`] type, allowing to
/// notify and convert (to [`ExitCode`]) any jsmoke output type.
pub type RuntimeOutput = Result<Box<dyn SuccessConstraint>, Box<dyn FailureConstraint>>;

/// Trait that works similar to [`Into<ExitCode>`] but for foreign types ([`Result`], in that
/// case).
pub trait IntoExitCode {
    /// Converts the [`Self`] value into an [`ExitCode`].
    fn into_exit_code(self) -> ExitCode;
}

impl IntoExitCode for RuntimeOutput {
    fn into_exit_code(self) -> ExitCode {
        if self.is_ok() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }
}

/// Common trait for [`RuntimeOutput`] notifying (independent of [`Result`] variant).
pub trait OutputNotify {
    /// General notify function. Works as [`NotifySuccess`] for [`Ok`] variants and
    /// [`NotifyFailure`] for [`Err`] variants.
    fn output_notify(&self);
}

impl OutputNotify for RuntimeOutput {
    fn output_notify(&self) {
        match self {
            Ok(o) => o.notify_success(),
            Err(e) => e.notify_failure(),
        }
    }
}

impl Verbose for RuntimeOutput {
    fn print_verbose(&self) {
        match self {
            Ok(o) => {
                o.print_verbose();
            }
            Err(e) => {
                e.print_verbose();
            }
        }
    }
}
