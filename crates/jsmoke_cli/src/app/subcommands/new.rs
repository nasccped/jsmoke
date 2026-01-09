use crate::types::vcs::VersionControlSystem;
use clap::Parser;

/// Fields when calling the `new` subcommand.
#[derive(Parser)]
pub struct NewSubcommand {
    /// The project name (also used as path - set `--path` flag to
    /// override).
    pub name: Option<String>,

    /// The project group.
    #[arg(short, long)]
    pub group: Option<String>,

    /// The project description.
    #[arg(long)]
    pub description: Option<String>,

    /// The version to lock project (accepts regex).
    #[arg(short, long)]
    pub lock_version: Option<String>,

    /// Main func class-path (uses `<GROUP>.<NAME>` when
    /// not set).
    #[arg(short, long)]
    pub default_main: Option<String>,

    /// Path to place the new project being created (uses `[NAME]`
    /// when not set).
    #[arg(short, long)]
    pub path: Option<String>,

    /// Create parent dirs (if not exists).
    #[arg(short, long)]
    pub parent: bool,

    /// Authors of the project. Name is required (empty mail when not
    /// set).
    #[arg(short, long, value_name = "NAME <MAIL_ADDRESS>")]
    pub authors: Option<String>,

    /// Version control system to be used (available: git, hg, pijul,
    /// svn, none).
    #[arg(long, value_enum, hide_possible_values = true)]
    pub vcs: Option<VersionControlSystem>,
}
