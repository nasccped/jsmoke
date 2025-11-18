//! # Subcommands module
//!
//! This module provides access to the JSmoke CLI subcommands and
//! its fields.

use clap::Subcommand;

/// JSmoke subcommands groups.
#[derive(Subcommand)]
pub enum JsmkSubcommand {
    /// Create a new project within a new directory.
    New,
    /// Create a new project within the current directory.
    Init,
    /// Compile the current project's source code.
    Build,
    /// Run the current project's bytecode (build it if not exists/updated).
    Run,
    /// Clean the current project's output files.
    Clean,
}
