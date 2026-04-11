//! # Common module
//!
//! Common data types + implementing used during runtime.
mod project_artifact;
mod reserved_words;

// public usage
pub use project_artifact::Artifact;

// non public usage
use reserved_words::ReservedWords;
