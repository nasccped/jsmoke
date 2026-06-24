//! Subcommand related module.
mod new;

use clap::Subcommand;
pub use new::New;

/// All available subcommand.
#[derive(Subcommand, Debug)]
pub enum AppSubcommands {
    /// Creates a new project within a new directory.
    New(New),
}
