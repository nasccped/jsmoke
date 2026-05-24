mod artifact_regex;
mod errors;
mod input_checker;
mod wrapper;

/// Minimum length allowed to an artifact name.
const ARTIFACT_MIN_LENGTH: usize = 4;

/// Maximum length allowed to an artifact name.
const ARTIFACT_MAX_LENGTH: usize = 30;

pub use wrapper::ArtifactWrapper;
