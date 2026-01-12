//! # New subcommand
//!
//! Field definition for the new subcommand.
use clap::Args;

/// Subcommand responsible for create new projects within a new directory.
#[derive(Args, Debug)]
pub struct New {
    /// The name of the created project (CamelCase expected).
    name: Option<String>,
    /// Where to place the new project (same as `name` by default).
    #[arg(long, short = 'p')]
    path: Option<String>,
    /// Lock the project to the version regex.
    #[arg(long = "lock", short = 'l', value_name = "(>|=|>=|<|<=)VERSION")]
    lock_version: Option<String>,
    /// The author(s) of the project.
    #[arg(long, value_name = "NAME1<EMAIL1>,N2...")]
    authors: Option<String>,
    /// The description of the project.
    #[arg(long, value_name = "QUOTED")]
    description: Option<String>,
    /// The prefered version control system to be used (git as default).
    #[arg(long)]
    vcs: Option<String>,
    /// The artifact name of the created project (empty by default).
    #[arg(long, short = 'a')]
    artifact: Option<String>,
    /// The group name of the created project (empty by default).
    #[arg(long, short = 'g')]
    group: Option<String>,
    /// The package name of the created project (group+artifact by default).
    #[arg(long)]
    package: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    use clap::Parser;

    const NAME: &str = "somename";
    const PATH: &str = "mypath";
    const AUTHORS: &str = "nasccped <my@mail.com>, pednascc <rev@erse>";
    const DESCRIPTION: &str = "a cool testing case";
    const VCS: &str = "git";
    const LOCK: &str = ">19.3.4";
    const ARTIFACT: &str = "someproject";
    const GROUP: &str = "my.group";
    const PACKAGE: &str = "crust.package.project";

    #[derive(Parser, Debug)]
    struct NewTest {
        #[command(flatten)]
        new: New,
    }

    impl NewTest {
        /// Generates a [`New`] struct over items that can be iterate. Don't require an initial
        /// argument (i.e. empty string).
        fn parse_from_iter<'a, I: IntoIterator<Item = &'a str>>(iter: I) -> New {
            let mut v = vec![""];
            v.extend(iter);
            match Self::try_parse_from(v) {
                Ok(x) => x.new,
                Err(x) => panic!("error when parsing, received => {:?}", x),
            }
        }
    }

    #[test]
    fn longs() {
        let new = NewTest::parse_from_iter([
            NAME,
            "--path",
            PATH,
            "--lock",
            LOCK,
            "--artifact",
            ARTIFACT,
            "--authors",
            AUTHORS,
            "--vcs",
            VCS,
            "--description",
            DESCRIPTION,
            "--group",
            GROUP,
            "--package",
            PACKAGE,
        ]);
        assert!(new.name.is_some_and(|n| n == NAME));
        assert!(new.path.is_some_and(|n| n == PATH));
        assert!(new.lock_version.is_some_and(|n| n == LOCK));
        assert!(new.artifact.is_some_and(|n| n == ARTIFACT));
        assert!(new.group.is_some_and(|n| n == GROUP));
        assert!(new.package.is_some_and(|n| n == PACKAGE));
    }

    #[test]
    fn short() {
        let new =
            NewTest::parse_from_iter([NAME, "-p", PATH, "-l", LOCK, "-a", ARTIFACT, "-g", GROUP]);
        assert!(new.name.is_some_and(|n| n == NAME));
        assert!(new.path.is_some_and(|n| n == PATH));
        assert!(new.lock_version.is_some_and(|n| n == LOCK));
        assert!(new.artifact.is_some_and(|n| n == ARTIFACT));
        assert!(new.group.is_some_and(|n| n == GROUP));
        assert!(new.package.is_none());
    }

    #[test]
    fn empty() {
        let new = NewTest::parse_from_iter([]);
        assert!(new.name.is_none());
        assert!(new.path.is_none());
        assert!(new.lock_version.is_none());
        assert!(new.artifact.is_none());
        assert!(new.group.is_none());
        assert!(new.package.is_none());
    }
}
