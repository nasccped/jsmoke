//! # Computer Command
//!
//! Provides the [`ComputerCommand`] struct and it's implementations.
use crate::printing::warning_print::WarningPrint;
use jsmoke_macros::WarningPrint;
use std::process::{Command, Stdio};
use thiserror::Error;

/// Representation of a computer command (similar to
/// [`std::process::Command`]) but provides testing abstraction.
pub struct ComputerCommand {
    /// Command name.
    pub name: String,
    /// Args to be ran.
    pub args: Vec<String>,
}

/// Possible errors when running [`ComputerCommand`].
#[derive(WarningPrint, Error, Debug)]
pub enum ComputerCommandError {
    /// The command fails 'cause the it doesn't exists.
    #[error("the provided command doesn't exists (`{0}`)")]
    DontExists(String),
    /// The command exists but fails with the provided args.
    #[error(
        "the provided command returned `1` (`{}: {}`)",
        .name,
        .args.join(" ")
    )]
    ErrorStatusReturned {
        /// Command name.
        name: String,
        /// Args that were used.
        args: Vec<String>,
        /// Stderr message.
        stderr: String,
    },
}

/// Representation of a successfully ran command.
#[allow(dead_code)]
pub struct ComputerCommandOutput {
    /// Name of the command.
    name: String,
    /// It's args.
    args: Vec<String>,
    /// The Stdout message.
    output: String,
}

impl ComputerCommand {
    /// # Computer Command run.
    ///
    /// This function runs the [`ComputerCommand`] as a computer process.
    ///
    /// Returns a [`Result<ComputerCommandOutput, ComputerCommandError>`], which:
    /// - [`ComputerCommandOutput`] holds the command run data
    ///   (command name, command args and output message);
    /// - [`ComputerCommandError`] holds the error variant generated
    ///   by the command run ([`ComputerCommandError::DontExists`] /
    ///   [`ComputerCommandError::ErrorStatusReturned`]).
    ///
    /// # Note
    ///
    /// The `DontExists` doesn't check if the command really exists.
    /// Instead, it calls the [`Command::output`] function and checks
    /// if the return value is an [`Err`] variant.
    ///
    /// The same logic is applied to `ErrorStatusReturned`. The
    /// [`Command::output`] is called and then, checks if the status
    /// is not a success (commonly `1`).
    ///
    /// # Panics
    ///
    /// This function catches the [`ComputerCommandOutput::output`]
    /// and the [`ComputerCommandError::ErrorStatusReturned::stderr`]
    /// values by receiving it from the [`Command`] itself (a
    /// [`Vec<u8>`] type) and turning it into a [`String`] by using
    /// the [`String::from_utf8`] function.
    ///
    /// This function will return a [`Result<String, FromUtf8Error>`]
    /// type value, and an `utf8` str is expect, otherwise, the
    /// program will panic with the [`Result::expect`] action.
    pub fn run(&self) -> Result<ComputerCommandOutput, ComputerCommandError> {
        let (name, args) = (self.name.clone(), self.args.clone());
        match Command::new(&self.name)
            .args(&self.args)
            .stdout(Stdio::piped())
            .output()
        {
            Ok(c) if c.status.success() => {
                let output = String::from_utf8(c.stdout).expect("expecting valid utf o_O");
                Ok(ComputerCommandOutput { name, args, output })
            }
            Ok(c) => {
                let err: String = String::from_utf8(c.stdout).expect("expecting valid utf o_O");
                Err(ComputerCommandError::ErrorStatusReturned {
                    name,
                    args,
                    stderr: err,
                })
            }
            Err(_) => Err(ComputerCommandError::DontExists(name)),
        }
    }

    /// Push a new argument to the [`Self::args`] field and returns a
    /// mutable reference to itself (like builder pattern).
    pub fn push_arg(&mut self, arg: impl ToString) -> &mut Self {
        self.args.push(arg.to_string());
        self
    }
}
