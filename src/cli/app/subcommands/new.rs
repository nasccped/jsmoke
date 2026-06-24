use clap::Args;
use std::path::PathBuf;

/// New subcommand.
#[derive(Debug, Args)]
pub struct New {
    /// Artifact of the project being created (ignore with '--no-artifact').
    #[arg(value_name = "ART")]
    artifact: Option<String>,

    /// Final destination of the new project (`artifact` as default).
    #[arg(long, value_name = "P")]
    path: Option<PathBuf>,

    /// Ignore artifact requirement.
    #[arg(long)]
    no_artifact: bool,
}

#[cfg(test)]
mod tests {
    use super::New;
    use crate::cli::{App, app::subcommands::AppSubcommands};
    use clap::Parser;
    use std::path::Path;

    /// [`New`] struct wrapper for test purpose.
    struct NewTest(New);

    impl NewTest {
        /// Default arguments for [`App`] parsing.
        const DEFAULT_ARGS: [&str; 2] = ["jsmk", "new"];

        /// Create a [`NewTest`] struct with the provided args (no args for empty [`New`]
        /// building).
        #[allow(unreachable_patterns)]
        fn new<const N: usize>(args: [&str; N]) -> Self {
            let mut v = Vec::from(Self::DEFAULT_ARGS);
            v.extend(args);
            let subcmd = match App::parse_from(v).subcommand {
                AppSubcommands::New(new) => new,
                other => panic!("Expecting `new` subcommand but got {:?}", other),
            };
            Self(subcmd)
        }

        /// Returns an option over the [`New`]'s `artifact` field.
        fn artifact(&self) -> Option<&str> {
            self.0.artifact.as_deref()
        }

        /// Returns the [`New`]'s `no_artifact` field.
        fn no_artifact(&self) -> bool {
            self.0.no_artifact
        }

        /// Returns the [`New`]'s `path` field.
        fn path(&self) -> Option<&Path> {
            self.0.path.as_deref()
        }
    }

    #[test]
    fn artifact_and_no_artifact() {
        let mut new: NewTest;
        new = NewTest::new([]);
        assert!(new.artifact().is_none() && !new.no_artifact());
        new = NewTest::new(["my-artifact"]);
        assert_eq!(new.artifact(), Some("my-artifact"));
        assert!(!new.no_artifact());
        new = NewTest::new(["my-artifact", "--no-artifact"]);
        assert_eq!(new.artifact(), Some("my-artifact"));
        assert!(new.no_artifact());
    }

    #[test]
    fn path() {
        let mut new: NewTest;
        new = NewTest::new([]);
        assert_eq!(new.path(), None);
        new = NewTest::new(["--path", "my/cool/path"]);
        assert_eq!(new.path(), Some(Path::new("my/cool/path")));
    }
}
