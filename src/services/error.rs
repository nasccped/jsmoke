use super::new_service::NewServiceParseError;
use crate::utils::{Verbose, notifier::Notifier};
use std::{
    fmt::{Debug, Display},
    process::ExitCode,
};

/// Sealed error trait. Turn any [`Display`], [`Verbose`] and [`Debug`] implementor in a error that
/// can be held.
trait ErrorConstraint<'a>: Display + Verbose + Debug {}

impl<'a, T: Display + Verbose + Debug + 'a> ErrorConstraint<'a> for T {}

/// Error when trying to parse a [`crate::cli::subcommands::AppSubcommands`] variant into it's
/// service.
#[derive(thiserror::Error, Debug)]
#[error("{}", .err)]
pub struct ServiceParseError<'a> {
    /// If the operation was called as verbose.
    verbose: bool,

    /// Error value.
    err: Box<dyn ErrorConstraint<'a> + 'a>,
}

impl<'a> ServiceParseError<'a> {
    /// Returns if the called operation is verbose.
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    /// Updates the inner `verbose` field according to `verbose` param.
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    /// Notifies itself (auto-prints verbose if necessary).
    pub fn self_notify(&self) {
        let mut notifier = Notifier::from(self);
        notifier.notify_failure(self);
        if self.is_verbose() {
            notifier.notify_verbose(self.err.as_ref());
        }
    }
}

impl<'a> From<NewServiceParseError<'a>> for ServiceParseError<'a> {
    fn from(value: NewServiceParseError<'a>) -> Self {
        Self {
            verbose: false,
            err: Box::new(value),
        }
    }
}

impl From<ServiceParseError<'_>> for ExitCode {
    fn from(_: ServiceParseError<'_>) -> Self {
        ExitCode::FAILURE
    }
}
