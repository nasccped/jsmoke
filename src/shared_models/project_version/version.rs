use super::{error::ProjectVersionError, version_regex::VersionRegex};
use crate::utils::InputFix;
use std::str::FromStr;

/// A type that represents version values (`major`, `minor`, `patch`).
#[derive(Debug, Clone)]
pub struct VersionValues {
    major: usize,
    minor: Option<usize>,
    patch: Option<usize>,
}

impl VersionValues {
    /// Creates a [`VersionValues`] over expected values.
    fn new(major: usize, minor: Option<usize>, patch: Option<usize>) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Checks if the [`VersionValues`] is valid (inner fields).
    pub fn is_valid(&self) -> bool {
        let &Self {
            major,
            minor,
            patch,
        } = self;
        [major, minor.unwrap_or(0), patch.unwrap_or(0)]
            .into_iter()
            .any(|x| x > 0)
    }
}

#[derive(Debug, Clone)]
enum RangeKind {
    Inclusive,
    Exclusive,
}

#[derive(Debug, Clone, Default)]
enum SingleKind {
    #[default]
    EqualsOrGreater,
    StrictlyEquals,
}

impl FromStr for SingleKind {
    type Err = ProjectVersionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">=" => Ok(Self::EqualsOrGreater),
            "=" | "" => Ok(Self::StrictlyEquals),
            other => Err(ProjectVersionError::Constraint(other.into())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProjectVersion {
    Single {
        kind: SingleKind,
        version: VersionValues,
    },
    Range {
        kind: RangeKind,
        min: VersionValues,
        max: VersionValues,
    },
}

impl FromStr for ProjectVersion {
    type Err = ProjectVersionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.input_fix(|string| string.split_whitespace().collect::<Vec<_>>().join(" "));
        if let Some(_) = VersionRegex::may_single(&s) {
            todo!("`{}` is a valid regex (single kind)", s);
        } else if let Some(_) = VersionRegex::may_range(&s) {
            todo!("`{}` is a valid regex (range kind)", s);
        } else {
            Err(ProjectVersionError::Pattern(s))
        }
    }
}

#[cfg(test)]
mod test {
    use super::ProjectVersion;
    use std::str::FromStr;

    const VALIDS: &[&str] = &["1", "1.2", "1.2.4", "12.43.3243"];

    #[test]
    #[ignore = "temp ignore"]
    fn valids() {
        VALIDS.iter().for_each(|vers| {
            let res = ProjectVersion::from_str(vers);
            assert!(
                res.is_ok(),
                "`{}` was expected to result in ok but `{:?}` was returned",
                vers,
                res
            )
        });
    }
}
