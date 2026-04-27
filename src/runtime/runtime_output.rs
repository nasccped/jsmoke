use crate::utils::{
    notifiers::{NotifyFailure, NotifySuccess},
    verbose::Verbose,
};
use std::process::ExitCode;

/// Type alias for runtime possible results.
pub type RuntimeOutput = Result<Box<dyn SuccessConstraint>, Box<dyn FailureConstraint>>;

/// Utilities for [`RuntimeOutput`] type.
pub trait RuntimeOutputUtils {
    /// Get [`ExitCode`] from `self` item.
    fn get_exit_code(&self) -> ExitCode;

    /// Get [`Verbose`] item as `&dyn`.
    fn as_verbose(&self) -> &dyn Verbose;
}

impl RuntimeOutputUtils for RuntimeOutput {
    fn get_exit_code(&self) -> ExitCode {
        if self.is_ok() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }

    fn as_verbose(&self) -> &dyn Verbose {
        match self {
            Ok(success) => success.as_ref(),
            Err(fail) => fail.as_ref(),
        }
    }
}

/// Constraint for successfully result variants.
pub trait SuccessConstraint: Verbose + NotifySuccess {}

/// Constraint for failure result variants.
pub trait FailureConstraint: Verbose + NotifyFailure {}

type BoxedFailure = Box<dyn FailureConstraint + 'static>;

impl<T: FailureConstraint + 'static> From<T> for BoxedFailure {
    fn from(value: T) -> Self {
        Box::new(value) as BoxedFailure
    }
}

// auto impl
impl<T: NotifySuccess + Verbose> SuccessConstraint for T {}
impl<T: NotifyFailure + Verbose> FailureConstraint for T {}
