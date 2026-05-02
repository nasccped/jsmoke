//! # Common module
//!
//! Common data types + implementing used during runtime.
mod path_error;
mod project_artifact;
mod project_version;
mod project_path;
mod reserved_words;

pub use path_error::PathError;
pub use project_artifact::Artifact;
pub use project_path::ProjectPath;
pub use project_version::{VersionLiteral, LockVersion};
