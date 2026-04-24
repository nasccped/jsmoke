use crate::{
    utils::{
        notifiers::NotifyFailure,
        verbose::{Verbose, VerboseWrapper},
        visuals::style::CommandStyle,
    },
    verbose_wrapper,
};
use thiserror::Error;

/// Unit error when no subcommand is passed.
#[derive(Error, Debug)]
#[error("no subcommand given")]
pub struct NoSubcommand;

impl NotifyFailure for NoSubcommand {}

impl Verbose for NoSubcommand {
    fn as_verbose(&self) -> VerboseWrapper {
        verbose_wrapper!(
            "Consider using {}." => "jsmk --help".command_style();
        )
    }
}
