//! # Version Control System
//!
//! Provides the [`VersionControlSystem`] enum and it's common
//! implementing.
#![allow(clippy::upper_case_acronyms)]
use clap::ValueEnum;

/// Available VCSs for new projects.
#[derive(ValueEnum, Copy, Clone, PartialEq)]
pub enum VersionControlSystem {
    /// Init a Git repository.
    Git,
    /// Init a Mercurial repository.
    Hj,
    /// Init a Pijul repository.
    Pijul,
    /// Init a Subversion repository.
    SVN,
    /// New project without repository.
    None,
}
