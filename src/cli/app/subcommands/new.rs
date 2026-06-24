use clap::Args;
use std::path::{Path, PathBuf};

/// New subcommand.
#[derive(Debug, Args)]
pub struct New {
    /// Artifact of the project being created (ignore with '--no-artifact').
    #[arg(value_name = "ART")]
    project_artifact: Option<String>,

    /// Final destination of the new project (`artifact` as default).
    #[arg(long = "path", value_name = "P")]
    project_path: Option<PathBuf>,

    /// Ignore artifact requirement.
    #[arg(long = "no-artifact")]
    ignore_artifact: bool,
}

impl New {
    /// Returns the optional artifact field.
    pub fn artifact(&self) -> Option<&str> {
        self.project_artifact.as_deref()
    }

    /// Returns the optional path destination.
    pub fn path(&self) -> Option<&Path> {
        self.project_path.as_deref()
    }

    /// If the `--no-artifact` flag is toggle on.
    pub fn no_artifact(&self) -> bool {
        self.ignore_artifact
    }
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

        /// Asserts if the self artifact is equals to the `expected` param.
        fn assert_artifact(&self, expected: Option<&str>) -> &Self {
            assert_eq!(self.0.artifact(), expected);
            self
        }

        /// Asserts if the self artifact is equals to the `expected` param.
        fn assert_no_artifact(&self, expected: bool) -> &Self {
            assert!(self.0.no_artifact() == expected);
            self
        }

        /// Asserts if the self path is equals to the `expected` param.
        fn assert_path(&self, expected: Option<&str>) -> &Self {
            assert_eq!(self.0.path(), expected.map(Path::new));
            self
        }
    }

    #[test]
    fn new_subcommand() {
        NewTest::new([])
            .assert_artifact(None)
            .assert_path(None)
            .assert_no_artifact(false);
        NewTest::new(["my-project"])
            .assert_artifact(Some("my-project"))
            .assert_path(None)
            .assert_no_artifact(false);
        NewTest::new(["my-project", "--path", "other/path"])
            .assert_artifact(Some("my-project"))
            .assert_path(Some("other/path"))
            .assert_no_artifact(false);
        NewTest::new(["--no-artifact"]).assert_no_artifact(true);
    }
}
