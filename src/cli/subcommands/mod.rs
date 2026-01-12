//! # Subcommands
//!
//! Provides the main subcommand enumerator + inner constraints for each variant.
mod new;

use clap::Subcommand as ClapSubcommand;
pub use new::New;

/// Subcommand variants for jsmoke cli app.
#[derive(ClapSubcommand, Debug)]
pub enum Subcommand {
    /// Build the bytecode of the current jsmoke project.
    Build,
    /// Clear the generate bytecode from the current jsmoke project.
    Clean,
    /// Initialize a new project in the current directory.
    Init,
    /// Creates a new project within a new directory.
    New(Box<New>),
    /// Runs the current jsmoke project.
    Run,
}
