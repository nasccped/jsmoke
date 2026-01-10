//! # New subcommand
//!
//! Field definition for the new subcommand.
use clap::Args;

/// Subcommand responsible for create new projects within a new directory.
#[derive(Args, Debug)]
pub struct New {
    /// The name of the created project (CamelCase expected).
    name: Option<String>,
    /// Lock the project to the version regex.
    #[arg(long = "lock", short = 'l', value_name = "(>|=|>=|<|<=)VERSION")]
    lock_version: Option<String>,
    /// The artifact name of the created project (empty by default).
    #[arg(long, short = 'a')]
    artifact: Option<String>,
    /// The group name of the created project (empty by default).
    #[arg(long, short = 'g')]
    group: Option<String>,
    /// The package name of the created project (group+artifact by default).
    #[arg(long, short = 'p')]
    package: Option<String>,
}
