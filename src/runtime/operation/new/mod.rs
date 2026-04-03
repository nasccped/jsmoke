use super::{
    super::{IntoOk, RuntimeOutput, common::project_name::ProjectName, output::FailureConstraint},
    OperationTrait,
};
use crate::{cli::subcommands::New as NewSubcommand, utils::Verbose};
use std::fmt::Display;

/// Operation related to the [`NewSubcommand`]. It stores the `new` subcommand fields
/// (post-parsing) and runs itself based on its inner fields.
pub struct NewOperation {
    /// The project name.
    name: ProjectName,
}

/// Type used to report when the operation succeeds.
pub struct NewOperationSuccess {
    /// The project name.
    name: ProjectName,
}

impl Display for NewOperationSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` successfuly created!", self.name)
    }
}

impl Verbose for NewOperationSuccess {
    fn print_verbose(&self) {
        println!("Other data");
    }
}

impl TryFrom<Box<NewSubcommand>> for NewOperation {
    type Error = Box<dyn FailureConstraint>;
    fn try_from(value: Box<NewSubcommand>) -> Result<Self, Self::Error> {
        let name = ProjectName::try_from(value.name)?;
        Ok(Self { name })
    }
}

impl OperationTrait for NewOperation {
    fn run(self, _: bool, _: bool) -> RuntimeOutput {
        (NewOperationSuccess { name: self.name }).into_ok()
    }
}
