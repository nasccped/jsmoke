use crate::{
    utils::{
        notifiers::NotifyFailure,
        regex::{EMPTY_REGEX, NEW_REGEX_WITH, WHITESPACE_REGEX},
        verbose::{Verbose, VerboseWrapper},
        visuals::style::CommandStyle,
    },
    verbose_wrapper,
};
use std::{ffi::OsStr, path::Path, path::PathBuf};
use thiserror::Error as ThisError;

/// Possible errors when parsing a project path.
#[derive(ThisError, Debug, PartialEq)]
pub enum ProjectPathError {
    /// When the passed path is an empty string.
    #[error("path can't be empty string")]
    Empty,
    /// When the passed path is a whitespace string.
    #[error("path can't be whitespace string")]
    WhiteSpace,
    /// When the path contains relative components (`.` or `..`).
    #[error("path can't be relative ('.' or '..')")]
    Relative(Box<Path>),
    /// When the path contains multi components (ie: `some/nested/path`).
    #[error("path can't be `multi-component` ({})", {
        let mut p = PathBuf::new();
        p.push(parents);
        p.push(child.as_ref());
        p.to_string_lossy().into_owned()
    })]
    MultiComponents {
        /// Parent to the child.
        parents: Box<Path>,
        /// Child itself.
        child: Box<OsStr>,
    },
}

impl NotifyFailure for ProjectPathError {}

impl ProjectPathError {
    /// Returns [`ProjectPathError::Empty`] variant if it matches regex.
    pub fn empty_if_matches<T: AsRef<str>>(s: T) -> Option<Self> {
        EMPTY_REGEX.is_match(s.as_ref()).then_some(Self::Empty)
    }

    /// Returns [`ProjectPathError::WhiteSpace`] variant if it matches regex.
    pub fn whitespace_if_matches<T: AsRef<str>>(s: T) -> Option<Self> {
        WHITESPACE_REGEX
            .is_match(s.as_ref())
            .then_some(Self::WhiteSpace)
    }

    /// Returns [`ProjectPathError::Relative`] variant if it matches regex.
    pub fn relative_if_matches<T: AsRef<str>>(s: T) -> Option<Self> {
        let s = s.as_ref();
        NEW_REGEX_WITH(r#"^(.*\/)*[.]+(\/.*)*$"#)
            .is_match(s)
            .then_some(Self::Relative(Path::new(s).into()))
    }

    /// Returns [`ProjectPathError::MultiComponents`] variant if it trigger on [`Path`] detection
    /// rules.
    pub fn multicomponent_if_detected<T: AsRef<str>>(s: T) -> Option<Self> {
        let s = s.as_ref();
        let mut parts = s.split(['/', '\\']).collect::<Vec<_>>().into_iter();
        let child = parts.next_back().map(Path::new);
        let parents: Option<PathBuf> = parts.fold(None, |accum: Option<PathBuf>, part| {
            if let Some(mut buf) = accum {
                buf.push(part);
                Some(buf)
            } else {
                Some(PathBuf::from(part))
            }
        });
        match (parents, child) {
            (Some(p), Some(c)) => Some(Self::MultiComponents {
                parents: p.into(),
                child: c.as_os_str().into(),
            }),
            _ => None,
        }
    }
}

impl Verbose for ProjectPathError {
    fn as_verbose(&self) -> VerboseWrapper {
        match self {
            Self::Relative(_) => verbose_wrapper!(
                "If you're trying to create a project at the curdir, consider";
                "using {} instead!" => "jsmk init".command_style();
            ),
            Self::MultiComponents { parents, child } => {
                let par = parents.to_string_lossy();
                verbose_wrapper!(
                    "Instead, consider creating the parent dirs, and then,";
                    "initializing the project:";
                    "";
                    "{} followed by {}" =>
                        format!("mkdir -p '{}' && cd '{}'", par, par).command_style(),
                        format!("jsmk new {}", child.display()).command_style()
                )
            }
            _ => verbose_wrapper!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Creates [`ProjectPathError::Empty`] option.
    fn e(s: &str) -> Option<ProjectPathError> {
        ProjectPathError::empty_if_matches(s)
    }

    /// Creates [`ProjectPathError::WhiteSpace`] option.
    fn w(s: &str) -> Option<ProjectPathError> {
        ProjectPathError::whitespace_if_matches(s)
    }

    /// Creates [`ProjectPathError::Relative`] option.
    fn r(s: &str) -> Option<ProjectPathError> {
        ProjectPathError::relative_if_matches(s)
    }

    /// Creates [`ProjectPathError::MultiComponents`] option.
    fn m(s: &str) -> Option<ProjectPathError> {
        ProjectPathError::multicomponent_if_detected(s)
    }

    #[test]
    fn empty() {
        assert_eq!(e(""), Some(ProjectPathError::Empty));
    }

    #[test]
    fn whitespace() {
        let target = Some(ProjectPathError::WhiteSpace);
        let inputs = [" ", " \t ", " \n "];
        inputs.into_iter().for_each(|i| {
            let res = w(i);
            assert_eq!(res, target, "expecting `{:?}` but got `{:?}`", target, res);
        })
    }

    #[test]
    fn relative() {
        let inputs = [
            "../start",
            "./start",
            "end/..",
            "end/.",
            "middle/../thing",
            "middle/./thing",
            "..",
            ".",
        ];
        inputs.into_iter().for_each(|i| {
            let res = r(i);
            assert!(
                matches!(res, Some(ProjectPathError::Relative(_))),
                "expecting Some of `ProjectPathError::Relative(_)` for '{}' but got `{:?}`",
                i,
                res
            );
        })
    }

    #[test]
    fn multicomponent() {
        let inputs = [
            "/start",
            "end/",
            "double//relative",
            "middle/thing",
            "tri/ni/ty",
        ];
        inputs.into_iter().for_each(|i| {
            let res = m(i);
            assert!(
                matches!(res, Some(ProjectPathError::MultiComponents { .. })),
                "expecting Some of `ProjectPathError::MultiComponents {{ .. }}` for '{}' but got `{:?}`",
                i,
                res
            );
        })
    }
}
