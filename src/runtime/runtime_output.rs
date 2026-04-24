use crate::utils::{
    notifiers::{NotifyFailure, NotifySuccess},
    verbose::Verbose,
};
use std::process::ExitCode;

/// Utilities for [`RuntimeOutput`] type.
pub trait RuntimeOutputUtils {
    /// Get [`ExitCode`] from `self` item.
    fn get_exit_code(&self) -> ExitCode;

    /// Get [`Verbose`] as boxed item from `self`.
    fn as_boxed_verbose(&self) -> Box<dyn Verbose>;
}

impl RuntimeOutputUtils for RuntimeOutput {
    fn get_exit_code(&self) -> ExitCode {
        if self.is_ok() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }

    fn as_boxed_verbose(&self) -> Box<dyn Verbose> {
        match self {
            Ok(success) => success.into(),
            Err(fail) => fail.into(),
        }
    }
}

/// Type alias for runtime possible results.
pub type RuntimeOutput = Result<Box<dyn SuccessConstraint>, Box<dyn FailureConstraint>>;

/// Constraint for successfully result variants.
pub trait SuccessConstraint: NotifySuccess + Verbose {}

/// Constraint for failure result variants.
pub trait FailureConstraint: NotifyFailure + Verbose {}

// auto impl
impl<T: NotifySuccess + Verbose> SuccessConstraint for T {}
impl<T: NotifyFailure + Verbose> FailureConstraint for T {}
