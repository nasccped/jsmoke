use crate::utils::SurelyUnwrap;
use regex::Regex;
use std::sync::LazyLock;

/// [`Regex`] struct for allowed [`super::ArtifactWrapper`] inputs.
static ALLOWED_ARTIFACT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[a-z][\w]*$"#).surely_unwrap());

/// [`Regex`] struct for fixable [`super::ArtifactWrapper`] inputs.
static FIXABLE_ARTIFACT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[a-z][\w]*(?:\-[\w]+)*$"#).surely_unwrap());

/// Struct for [`super::ArtifactWrapper`] matching check.
pub struct ArtifactRegex;

impl ArtifactRegex {
    /// If the provided artifact [`&str`] is allowed.
    pub fn is_allowed(artifact: &str) -> bool {
        ALLOWED_ARTIFACT_REGEX.is_match(artifact)
    }

    /// If the provided artifact [`&str`] is fixable.
    pub fn is_fixable(artifact: &str) -> bool {
        FIXABLE_ARTIFACT_REGEX.is_match(artifact)
    }
}
