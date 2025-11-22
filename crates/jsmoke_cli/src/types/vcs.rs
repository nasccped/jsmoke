//! # Version Control System
//!
//! Provides the [`VersionControlSystem`] enum and it's common
//! implementing.
#![allow(clippy::upper_case_acronyms)]
use clap::ValueEnum;
use jsmoke_utils::{computer_command::ComputerCommand, may::MayFrom};

const GIT_CMD: &str = "git";
const HG_CMD: &str = "hg";
const PIJUL_CMD: &str = "pijul";
const SVN_CMD: &str = "svnadmin";

/// Arg used by subversion.
const CREATE_ARG: &str = "create";
/// Arg used by git and others.
const INIT_ARG: &str = "init";

/// Available VCSs for new projects.
#[derive(ValueEnum, Copy, Clone, PartialEq, Debug)]
pub enum VersionControlSystem {
    /// Init a Git repository.
    Git,
    /// Init a Mercurial repository.
    Hg,
    /// Init a Pijul repository.
    Pijul,
    /// Init a Subversion repository.
    Svn,
    /// New project without repository.
    None,
}

impl MayFrom<&VersionControlSystem> for ComputerCommand {
    fn may_from(value: &VersionControlSystem) -> Option<Self> {
        let name: String = match value {
            VersionControlSystem::None => None,
            VersionControlSystem::Git => Some(GIT_CMD),
            VersionControlSystem::Hg => Some(HG_CMD),
            VersionControlSystem::Pijul => Some(PIJUL_CMD),
            VersionControlSystem::Svn => Some(SVN_CMD),
        }
        .map(|c| c.into())?;
        let arg: String = match value {
            VersionControlSystem::Svn => CREATE_ARG,

            // `None` variant doesn't need to be checked since it was
            // already eliminated in previous match.
            _ => INIT_ARG,
        }
        .into();
        let args = Vec::from([arg]);
        Some(Self { name, args })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        [
            ("git", VersionControlSystem::Git),
            ("hg", VersionControlSystem::Hg),
            ("pijul", VersionControlSystem::Pijul),
            ("svn", VersionControlSystem::Svn),
            ("none", VersionControlSystem::None),
        ]
        .iter()
        .for_each(|(cmd, vcs)| assert_eq!(VersionControlSystem::from_str(cmd, true), Ok(*vcs)));
    }

    #[test]
    fn name_and_arg() {
        [
            (VersionControlSystem::Git, GIT_CMD, INIT_ARG),
            (VersionControlSystem::Hg, HG_CMD, INIT_ARG),
            (VersionControlSystem::Pijul, PIJUL_CMD, INIT_ARG),
            (VersionControlSystem::Svn, SVN_CMD, CREATE_ARG),
        ]
        .into_iter()
        .for_each(|(vcs, cmd, arg)| {
            let cc = ComputerCommand::may_from(&vcs).expect("this was expected to be some o_O");
            let (name, args) = tuple(&cc);
            let first_arg = args
                .iter()
                .next()
                .expect("the computer command was expected to hold at least one arg")
                .as_ref();
            assert_eq!((name, first_arg), (cmd, arg));
        });
    }

    #[test]
    fn none_computer_command() {
        assert!(ComputerCommand::may_from(&VersionControlSystem::None).is_none());
    }

    /// Returns the [`ComputerCommand`] fields as a tuple of
    /// references.
    fn tuple(cc: &ComputerCommand) -> (&str, &[String]) {
        (&cc.name, &cc.args)
    }
}
