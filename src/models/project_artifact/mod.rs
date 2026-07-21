mod artifact;
mod error;
mod post_conversion_checker;
mod pre_conversion_checker;

use regex::Regex;
use std::sync::LazyLock;

pub use artifact::ProjectArtifact;
pub use error::ProjectArtifactParseError;

/// The [`Regex`] used for valid [`ProjectArtifact`] matching.
static PROJECT_ARTIFACT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[a-zA-Z][\w]*(?:-[a-zA-Z][\w]+)*$"#).unwrap());

/// The minimum str length for a [`ProjectArtifact`] item (counts after-fix chars only).
const PROJECT_ARTIFACT_MINIMUM_LENGTH: usize = 4;

/// The maximum str length for a [`ProjectArtifact`] item (counts after-fix chars only).
const PROJECT_ARTIFACT_MAXIMUM_LENGTH: usize = 30;
