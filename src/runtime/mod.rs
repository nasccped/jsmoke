mod common;
mod operation;
mod output;

use crate::{
    cli::{App, subcommands::Subcommand},
    runtime::{
        operation::OperationTrait,
        output::{OutputNotify, RuntimeOutput},
    },
    utils::{Verbose, visuals::style::CommandStyle},
};
pub use output::{IntoErr, IntoExitCode, IntoOk};
use std::process::ExitCode;
use thiserror::Error as ThisError;

/// Unit error when no subcommand is passed.
#[derive(ThisError, Debug)]
#[error("no subcommand given")]
struct NoSubcommand;

impl Verbose for NoSubcommand {
    fn print_verbose(&self) {
        eprintln!("Consider using {}", "jsmk --help".command_style());
    }
}

/// Run the jsmoke program based on the [`App`] inner fields.
pub fn run(app: App) -> ExitCode {
    let (subcommand, out, force, verbose): (Subcommand, RuntimeOutput, bool, bool);
    (force, verbose) = (app.force, app.verbose);
    match app.subcommand {
        Some(s) => subcommand = s,
        None => {
            out = NoSubcommand.into_err();
            out.output_notify();
            if verbose {
                out.print_verbose();
            }
            return out.into_exit_code();
        }
    }
    out = match subcommand {
        Subcommand::New(x) => {
            operation::NewOperation::try_from(x).and_then(|o| o.run(force, verbose))
        }
        other => unreachable!("this subcommand was called: {:?}", other),
    };
    out.output_notify();
    if verbose {
        out.print_verbose();
    }
    out.into_exit_code()
}
