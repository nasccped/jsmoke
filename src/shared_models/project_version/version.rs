use super::{
    error::ProjectVersionError,
    version_regex::{
        ConstraintGetter, RangeVersionCaptures, SingleVersionCaptures, VersionCaptures,
        VersionRegex,
    },
};
use crate::utils::InputFix;
use std::{cmp::Ordering, fmt::Display, str::FromStr};

/// A type that represents version values (`major`, `minor`, `patch`).
#[derive(Debug, Clone)]
pub struct VersionValues {
    major: usize,
    minor: Option<usize>,
    patch: Option<usize>,
}

impl VersionValues {
    const DEFAULT_FIELD_VALUE: usize = 0;
    fn as_array(&self) -> [usize; 3] {
        [
            self.major,
            self.minor.unwrap_or(Self::DEFAULT_FIELD_VALUE),
            self.patch.unwrap_or(Self::DEFAULT_FIELD_VALUE),
        ]
    }
}

impl PartialEq for VersionValues {
    fn eq(&self, other: &Self) -> bool {
        let left_iter = self.as_array().into_iter();
        let right_iter = other.as_array().into_iter();
        left_iter.zip(right_iter).all(|(left, right)| left == right)
    }
}

impl PartialOrd for VersionValues {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result =
            self.as_array()
                .into_iter()
                .zip(other.as_array())
                .fold(None, |dummy, (left, right)| match dummy {
                    x if x.is_some() => x,
                    _ if left < right => Some(Ordering::Less),
                    _ if left > right => Some(Ordering::Greater),
                    other => other,
                });
        if result.is_none() {
            Some(Ordering::Equal)
        } else {
            result
        }
    }
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

/// Different constraint for single kind version reprs.
#[derive(Debug, Clone, Default)]
pub enum SingleKind {
    /// When the version must be equals or greater.
    #[default]
    EqualsOrGreater,
    /// When the version must be strictly equals.
    StrictlyEquals,
}

impl SingleKind {
    /// Constraint sign when it's [`SingleKind::StrictlyEquals`].
    pub const STRICTLY_EQUALS_SIGN: &str = "=";

    /// Constraint sign when it's [`SingleKind::EqualsOrGreater`].
    pub const EQUALS_OR_GREATER_SIGN: &str = ">=";
}

impl Display for SingleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EqualsOrGreater => Self::EQUALS_OR_GREATER_SIGN,
                Self::StrictlyEquals => Self::STRICTLY_EQUALS_SIGN,
            }
        )
    }
}

impl FromStr for SingleKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            x if x == Self::EQUALS_OR_GREATER_SIGN || x.is_empty() => Ok(Self::EqualsOrGreater),
            x if x == Self::STRICTLY_EQUALS_SIGN => Ok(Self::StrictlyEquals),
            _other => Err(()),
        }
    }
}

/// Different constraint for range kind version reprs.
#[derive(Debug, Clone)]
pub enum RangeKind {
    /// When it's inclusive (includes the right side version).
    Inclusive,
    /// When it's exclusive (excludes the right side version).
    Exclusive,
}

impl RangeKind {
    /// Constraint sign when it's [`RangeKind::Inclusive`].
    pub const INCLUSIVE_SIGN: &str = "..=";

    /// Constraint sign when it's [`RangeKind::Exclusive`].
    pub const EXCLUSIVE_SIGN: &str = "..";
}

impl Display for RangeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Inclusive => Self::INCLUSIVE_SIGN,
                Self::Exclusive => Self::EXCLUSIVE_SIGN,
            }
        )
    }
}

impl FromStr for RangeKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            x if x == Self::INCLUSIVE_SIGN => Ok(Self::Inclusive),
            x if x == Self::EXCLUSIVE_SIGN => Ok(Self::Exclusive),
            _other => Err(()),
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

impl ProjectVersion {
    /// Creates a [`ProjectVersion::Single`] variant.
    fn new_single(kind: SingleKind, vals: VersionValues) -> Self {
        Self::Single {
            kind,
            version: vals,
        }
    }

    /// Creates a [`ProjectVersion::Single`] variant.
    fn new_range(kind: RangeKind, min: VersionValues, max: VersionValues) -> Self {
        Self::Range { kind, min, max }
    }

    fn is_valid(&self) -> bool {
        match self {
            Self::Single { version, .. } => version.is_valid(),
            Self::Range { min, max, .. } => min.is_valid() && max.is_valid() && min < max,
        }
    }
}

impl FromStr for ProjectVersion {
    type Err = ProjectVersionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // turn into a 'regular' string input
        let s = s.input_fix(|string| string.split_whitespace().collect::<Vec<_>>().join(" "));
        // if it's an error, early return it, otherwise, handle cases (pass it to the local
        // captures handler function)
        let re_captures = VersionRegex::try_from(s.as_str())
            .map_err(|string| ProjectVersionError::Pattern(string.into()))?;
        match re_captures {
            VersionRegex::Single(single_captures) => handle_single_captures(single_captures),
            VersionRegex::Range(range_captures) => handle_range_captures(range_captures),
        }
    }
}

/// Type alias for `handle` kind functions.
type ProjectVersionResult = Result<ProjectVersion, ProjectVersionError>;

/// Handle the [`SingleVersionCaptures`] input to return a [`ProjectVersion`]. Returns
/// [`ProjectVersionError`] if fails.
fn handle_single_captures(captures: SingleVersionCaptures<'_>) -> ProjectVersionResult {
    let as_str = captures.get_match().as_str();
    let kind = handle_constraint_kind::<SingleVersionCaptures, SingleKind>(&captures)
        .map_err(|e| e.set_inner(as_str))?
        .unwrap_or_default();
    let version = handle_version(captures.get_version()).map_err(|e| e.set_inner(as_str))?;
    // Ok only if valid
    match ProjectVersion::new_single(kind, version) {
        x if x.is_valid() => Ok(x),
        _ => Err(ProjectVersionError::Invalid(as_str.into())),
    }
}

/// Handle the [`RangeVersionCaptures`] input to return a [`ProjectVersion`]. Returns
/// [`ProjectVersionError`] if fails.
fn handle_range_captures(captures: RangeVersionCaptures<'_>) -> ProjectVersionResult {
    let as_str = captures.get_match().as_str();
    let set_inner = |err: ProjectVersionError| err.set_inner(as_str);
    let kind = handle_constraint_kind::<RangeVersionCaptures, RangeKind>(&captures)
        .map_err(set_inner)?
        .ok_or(ProjectVersionError::Pattern(as_str.into()))?;
    let min = handle_version(captures.get_left()).map_err(set_inner)?;
    let max = handle_version(captures.get_right()).map_err(set_inner)?;
    match ProjectVersion::new_range(kind, min, max) {
        x if x.is_valid() => Ok(x),
        _ => Err(ProjectVersionError::Invalid(as_str.into())),
    }
}

/// Type alias for [`handle_constraint_kind`] function return.
type ConstraintResult<T> = Result<Option<T>, ProjectVersionError>;

// Use alias for `handle_constraint_kind`.
use self::ConstraintGetter as CG;

/// Handles the [`ConstraintGetter`] implementor. Note that this function may return [`Ok`] with
/// [`None`]. This was made to fit the [`SingleVersionCaptures`] which can (or not) return a
/// constraint.
///
/// Notes:
/// 1. do 'sure handling' on the function caller.
/// 2. map [`Err`] using [`ProjectVersionError::set_inner`] (this function returns an empty string
///    as inner field).
fn handle_constraint_kind<'a, C: CG<'a>, Out: FromStr>(captures: &'a C) -> ConstraintResult<Out> {
    let constraint = match captures.get_constraint() {
        Some(c) => c,
        None => return Ok(None),
    };
    Out::from_str(constraint)
        .map(Some)
        // NOTE: the err can constains an empty `String`, just make sure to use `self.set_inner`
        // function to override it with the higher level string input
        .map_err(|_| ProjectVersionError::Pattern(String::new()))
}

/// Handles the [`VersionCaptures`] to return a [`VersionValues`]. Returns [`ProjectVersionError`]
/// if fails.
///
/// Note that this function doesn't runs the [`VersionValues::is_valid`] since it must be ran at
/// caller function.
fn handle_version(captures: VersionCaptures<'_>) -> Result<VersionValues, ProjectVersionError> {
    let as_str = captures.get_match().as_str();
    let parse_or_err = |val: &str| {
        val.parse::<usize>()
            .map_err(|_| ProjectVersionError::Parse(as_str.into()))
    };
    let get_optional_or_err = |val: Option<&str>| match val {
        Some(v) => parse_or_err(v).map(Some),
        _ => Ok(None),
    };
    let major = parse_or_err(captures.get_major())?;
    let minor = get_optional_or_err(captures.get_minor())?;
    let patch = get_optional_or_err(captures.get_patch())?;
    Ok(VersionValues::new(major, minor, patch))
}

#[cfg(test)]
mod test {
    use super::ProjectVersion;
    use std::str::FromStr;

    const VALIDS: &[&str] = &["1", "1.2", "1.2.4", "12.43.3243"];

    #[test]
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
