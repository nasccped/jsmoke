use crate::{
    cli::subcommands::New as NewSubcommand,
    runtime::{
        Context,
        common::{Artifact, LockVersion, ProjectPath},
        operations::OperationTrait,
        runtime_output::{FailureConstraint, RuntimeOutput},
    },
};

/// Operation related to the [`NewSubcommand`]. It stores the `new` subcommand fields
/// (post-parsing) and runs itself based on its inner fields.
pub struct New {
    /// The project name.
    artifact: Artifact,
    /// Path to place the project.
    path: ProjectPath,
    /// Lock version being used.
    lock: Option<LockVersion>,
}

impl TryFrom<NewSubcommand> for New {
    type Error = Box<dyn FailureConstraint>;
    fn try_from(value: NewSubcommand) -> Result<Self, Self::Error> {
        let artifact = Artifact::try_from_optional_string(value.artifact)?;
        let path = ProjectPath::from_path_or_artifact(value.path, &artifact)?;
        let mut lock: Option<LockVersion> = None;
        if let Some(l) = value.lock_version {
            lock = Some(LockVersion::try_from(l.as_str())?);
        }
        Ok(Self {
            artifact,
            path,
            lock,
        })
    }
}

impl OperationTrait for New {
    fn run(&self, ctx: Context) -> RuntimeOutput {
        let _ = ctx;
        // let _artifact = self.artifact;
        todo!("impl it");
        // let path = self.path.artifact_comp(&artifact);
        // Ok(NewSuccess::new(artifact, self.path))
    }
}
