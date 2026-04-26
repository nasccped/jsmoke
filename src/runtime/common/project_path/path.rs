use super::{super::Artifact, ProjectPathError};
use crate::utils::visuals::style::PathStyle;
use std::{fmt::Display, path::Path};

/// Struct wrapper for project path data.
#[derive(Debug, PartialEq)]
pub struct ProjectPath(Box<Path>);

impl ProjectPath {
    /// Returns the [`Path`] from the self item.
    pub fn get_path(&self) -> &Path {
        &self.0
    }

    /// Generates a [`ProjectPath`] result over an optional string. Otherwise, uses the
    /// [`Artifact`] fallback.
    pub fn from_path_or_artifact(
        path: Option<String>,
        artifact: &Artifact,
    ) -> Result<Self, ProjectPathError> {
        if let Some(p) = path {
            Self::try_from(p.as_str())
        } else {
            Ok(Self::from(artifact))
        }
    }
}

impl Display for ProjectPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_os_str().to_string_lossy().path_style())
    }
}

impl TryFrom<&str> for ProjectPath {
    type Error = ProjectPathError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = match ProjectPathError::empty_if_matches(value)
            .or(ProjectPathError::whitespace_if_matches(value))
        {
            Some(err) => Err(err),
            _ => Ok(value.split_whitespace().collect::<Vec<&str>>().join(" ")),
        }?;
        if let Some(err) = ProjectPathError::relative_if_matches(&value)
            .or(ProjectPathError::multicomponent_if_detected(&value))
        {
            Err(err)
        } else {
            Ok(Self(Path::new(&value).into()))
        }
    }
}

impl From<&Artifact> for ProjectPath {
    fn from(value: &Artifact) -> Self {
        Self(Path::new(value.get_view()).into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type ErrorMatchesType = (&'static str, fn(&ProjectPathError) -> bool);

    const ERROR_MATCHES: [ErrorMatchesType; 13] = [
        // separated by (input, expecting closure)
        ("", |val| matches!(val, ProjectPathError::Empty)),
        (" ", |val| matches!(val, ProjectPathError::WhiteSpace)),
        (" \n ", |val| matches!(val, ProjectPathError::WhiteSpace)),
        (" \t ", |val| matches!(val, ProjectPathError::WhiteSpace)),
        ("./path", |val| matches!(val, ProjectPathError::Relative(_))),
        ("../path", |val| {
            matches!(val, ProjectPathError::Relative(_))
        }),
        ("path/.", |val| matches!(val, ProjectPathError::Relative(_))),
        ("path/..", |val| {
            matches!(val, ProjectPathError::Relative(_))
        }),
        (".", |val| matches!(val, ProjectPathError::Relative(_))),
        ("..", |val| matches!(val, ProjectPathError::Relative(_))),
        ("com/pount", |val| {
            matches!(val, ProjectPathError::MultiComponents { .. })
        }),
        ("tri/ple/thing", |val| {
            matches!(val, ProjectPathError::MultiComponents { .. })
        }),
        ("empty//thing", |val| {
            matches!(val, ProjectPathError::MultiComponents { .. })
        }),
    ];

    const SUCCESS_MATCHES: [&str; 4] = ["some-path", "áccent", "with50numbers", "Capital"];

    /// Fast [`TryFrom`] call.
    fn tf(s: &str) -> Result<ProjectPath, ProjectPathError> {
        ProjectPath::try_from(s)
    }

    #[test]
    fn errors() {
        ERROR_MATCHES.into_iter().for_each(|(inp, matching)| {
            let err = tf(inp)
                .err()
                .unwrap_or_else(|| panic!("the '{}' input was expected to result into err", inp));
            assert!(matching(&err), "input '{}' results into `{:?}`.", inp, err,);
        });
    }

    #[test]
    fn successes() {
        SUCCESS_MATCHES.into_iter().for_each(|s| {
            let res = tf(s);
            assert!(res.is_ok(), "expecting Ok for '{}' but got `{:?}`", s, res);
        });
    }
}
