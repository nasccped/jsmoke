#[allow(unused_imports)]
use std::str::FromStr;

/// When [`super::ProjectVersion::from_str`] fails.
#[derive(thiserror::Error, Debug)]
pub enum ProjectVersionError {
    /// When the provided string doesn't follow the expected version regex pattern.
    #[error("not recognizable version string pattern ({})", .0)]
    Pattern(String),
    /// When the provided [`super::ProjectVersion`] kind constraint isn't valid.
    #[error("invalid constraint being used at version string ({})", .0)]
    Constraint(String),
    /// When the error happens at string parsing.
    #[error("couldn't parse the version string ({})", .0)]
    Parse(String),
}
