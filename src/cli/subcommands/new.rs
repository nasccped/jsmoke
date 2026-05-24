//! # New subcommand
//!
//! Field definition for the new subcommand.
use crate::shared_models::artifact_wrapper::ArtifactWrapper;
use clap::Args;
use std::path::PathBuf;

/// Subcommand responsible for create new projects within a new directory.
#[derive(Args, Debug)]
pub struct New {
    /// The artifact of the project being created.
    pub artifact: ArtifactWrapper,
    /// Where to place the new project (same as `artifact` by default).
    #[arg(long, short = 'p')]
    pub path: Option<PathBuf>,
    /// Lock the project to the version regex.
    #[arg(long = "lock", short = 'l', value_name = "(>= | =)VERSION")]
    pub lock_version: Option<String>,
    /// The author(s) of the project.
    #[arg(long, value_name = "NAME1<EMAIL1>,N2...")]
    pub authors: Option<String>,
    /// The description of the project.
    #[arg(long, value_name = "QUOTED")]
    pub description: Option<String>,
    /// The prefered version control system to be used (git as default).
    #[arg(long)]
    pub vcs: Option<String>,
    /// The group name of the created project (empty by default).
    #[arg(long, short = 'g')]
    pub group: Option<String>,
}
