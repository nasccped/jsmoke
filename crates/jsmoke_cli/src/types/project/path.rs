//! # Path module
//!
//! Holds the [`ProjectPath`] type + it's task implementing.
use std::path::PathBuf;

/// Type abstraction to represents a project path.
#[derive(Clone)]
pub struct ProjectPath {
    /// Path to place it.
    path: PathBuf,
    /// Create parents if not exists.
    parent: bool,
}

impl ProjectPath {
    /// Creates a new [`ProjectPath`] over a [`PathBuf`] (or any type
    /// that implements [`Into`] for it) + it's `parent` flag.
    fn new(path: impl Into<PathBuf>, parent: bool) -> Self {
        let path = path.into();
        Self { path, parent }
    }
}
