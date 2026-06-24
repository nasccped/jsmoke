//! Subcommand related module.
use clap::Subcommand;

/// All available subcommand.
#[derive(Subcommand, Debug)]
pub enum AppSubcommands {
    /// Does 'a' related things.
    AStuff,

    /// Does 'b' related things.
    BStuff,
}
