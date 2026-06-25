use crate::{models::project_artifact::ProjectArtifactParseError, utils::Verbose};

/// Different kind if errors when trying to parse [`crate::cli::subcommands::New`] to
/// [`super::NewService`].
#[derive(thiserror::Error, Debug)]
pub enum NewServiceParseError<'a> {
    /// The error was raise by the [`crate::models::project_artifact::ProjectArtifact::try_from`]
    /// function.
    #[error(transparent)]
    Artifact(ProjectArtifactParseError<'a>),
}

impl<'a> From<ProjectArtifactParseError<'a>> for NewServiceParseError<'a> {
    fn from(value: ProjectArtifactParseError<'a>) -> Self {
        Self::Artifact(value)
    }
}

impl<'a> Verbose for NewServiceParseError<'a> {
    fn get_verbose_message(&self) -> Option<std::borrow::Cow<'_, str>> {
        match self {
            Self::Artifact(art) => art.get_verbose_message(),
        }
    }
}
