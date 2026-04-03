use super::RuntimeOutput;
use crate::utils::{
    Verbose,
    printing::{NotifyFailure, NotifySuccess},
};

/// Trait for [`NotifySuccess`] `dyn` constraint (at [`RuntimeOutput`]).
pub trait SuccessConstraint: NotifySuccess + Verbose {}

/// Trait for [`NotifyFailure`] `dyn` constraint (at [`RuntimeOutput`]).
pub trait FailureConstraint: NotifyFailure + Verbose {}

// create itermediate trait impl to allow `Box<dyn NotifyError + Verbose>`.
impl<T: NotifySuccess + Verbose> SuccessConstraint for T {}
impl<T: NotifyFailure + Verbose> FailureConstraint for T {}

/// Trait for easy `success` to boxed [`Ok`] conversion.
pub trait IntoOk: SuccessConstraint {
    /// Converts any type that implements [`SuccessConstraint`] into
    /// `Ok(Box<dyn SuccessConstraint>)` (as expected by the RuntimeOutput signature).
    fn into_ok(self) -> RuntimeOutput;
}

/// Trait for easy `failure` to boxed [`Err`] conversion.
pub trait IntoErr: FailureConstraint {
    /// Converts any type that implements [`FailureConstraint`] into
    /// `Err(Box<dyn FailureConstraint>)` (as expected by the RuntimeOutput signature).
    fn into_err(self) -> RuntimeOutput;
}

impl<T: SuccessConstraint + 'static> IntoOk for T {
    fn into_ok(self) -> RuntimeOutput {
        Ok(Box::new(self))
    }
}

impl<E: FailureConstraint + 'static> IntoErr for E {
    fn into_err(self) -> RuntimeOutput {
        Err(Box::new(self))
    }
}

impl<T: FailureConstraint + 'static> From<T> for Box<dyn FailureConstraint> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
