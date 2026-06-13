#[allow(unused_imports)]
use std::str::FromStr;

/// When [`super::ProjectVersion::from_str`] fails.
#[derive(thiserror::Error, Debug)]
pub enum ProjectVersionError {
    /// When the provided string doesn't follow the expected version regex pattern.
    #[error("not recognizable version string pattern ({})", .0)]
    Pattern(String),
    /// When the error happens at string parsing.
    #[error("couldn't parse the version string ({})", .0)]
    Parse(String),
    /// When the error happens at `is_valid` function.
    #[error("version string contains invalid fields ({})", .0)]
    Invalid(String),
}

impl ProjectVersionError {
    /// Changes the inner [`String`] to the `value`'s [`String`] and returns the `self` item. Note
    /// that this function consumes `self`.
    pub fn set_inner(mut self, value: impl ToString) -> Self {
        let str_poiter = match &mut self {
            Self::Pattern(inner) => inner,
            Self::Parse(inner) => inner,
            Self::Invalid(inner) => inner,
        };
        *str_poiter = value.to_string();
        self
    }
}
