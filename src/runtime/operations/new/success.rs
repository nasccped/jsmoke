use crate::runtime::common::{Artifact, ProjectPath};

/// Type used to report when the [`super::New`] operation succeeds.
pub struct NewSuccess {
    /// The project's name.
    artifact: Artifact,
    /// The project's abspath.
    path: ProjectPath,
}

impl NewSuccess {
    pub fn new(artifact: Artifact, path: ProjectPath) -> Self {
        Self { artifact, path }
    }
}
