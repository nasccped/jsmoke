mod error;

use crate::{cli::subcommands::New as NewSubcommand, models::project_artifact::ProjectArtifact};
pub use error::NewServiceParseError;

/// Service for the `new` subcommand.
pub struct NewService<'a> {
    /// TEMP: The single purpose of this field is to allow lifetime hold (it'll be used latter).
    empty: Option<&'a str>,
}

impl<'a> TryFrom<&'a NewSubcommand> for NewService<'a> {
    type Error = NewServiceParseError<'a>;
    fn try_from(value: &'a NewSubcommand) -> Result<Self, Self::Error> {
        if let Some(art) = value.artifact() {
            let _ = ProjectArtifact::try_from(art.trim())?;
        }
        todo!("everythin goes well (so far)");
    }
}
