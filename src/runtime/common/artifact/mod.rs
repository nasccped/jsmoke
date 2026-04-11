//! # Artifact module
//!
//! Provides [`Artifact`], [`ArtifactError`] and it's public implementations.
#![allow(clippy::module_inception)]
mod artifact;
mod errors;

pub use artifact::Artifact;
pub use errors::ArtifactError;

/// Minimum length for an artifact name.
const MINIMUM_ARTIFACT_LENGTH: usize = 4;

/// Maximum length for an artifact name.
const MAXIMUM_ARTIFACT_LENGTH: usize = 50;
