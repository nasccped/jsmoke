use clap::{Arg, ArgAction, Command};

/// ´new` subcommand generator
pub fn subcommand() -> Command {
    Command::new("new")
        .about("Create a java app in a new directory")
        .args([
            arg_name(),
            arg_description(),
            arg_authors(),
            arg_path(),
            arg_vcs(),
            arg_source_path(),
            arg_out_path(),
            arg_lock_version(),
            arg_main_func(),
        ])
}

fn arg_name() -> Arg {
    Arg::new("name")
        .value_name("NAME")
        .action(ArgAction::Set)
        .required(true)
        .help("Specify the project name")
        .index(1)
}

fn arg_description() -> Arg {
    Arg::new("description")
        .long("description")
        .short('D')
        .action(ArgAction::Set)
        .value_name("QUOTED DESCRIPTION")
        .help("Add description to the project")
}

fn arg_authors() -> Arg {
    Arg::new("authors")
        .long("authors")
        .short('A')
        .action(ArgAction::Set)
        .value_name("QUOTED+COMMA SEPARATED")
        .help("Add project authors to toml file")
}

fn arg_path() -> Arg {
    Arg::new("path")
        .long("path")
        .short('P')
        .action(ArgAction::Set)
        .value_name("PATH")
        .help("Set an alternative path (<NAME> as default)")
}

fn arg_vcs() -> Arg {
    Arg::new("vcs")
        .long("vcs")
        .action(ArgAction::Set)
        .value_name("VCS")
        .help("Set a version control system [git, mercurial, pijul, fossil, none] (git as default)")
}

fn arg_source_path() -> Arg {
    Arg::new("source-path")
        .long("source-path")
        .short('S')
        .action(ArgAction::Set)
        .value_name("PATH")
        .help("Set the path that stores the source code ('src' as default)")
}

fn arg_out_path() -> Arg {
    Arg::new("out-path")
        .long("out-path")
        .short('O')
        .action(ArgAction::Set)
        .value_name("PATH")
        .help("Set the path that stores the code output ('out' as default)")
}

fn arg_lock_version() -> Arg {
    Arg::new("lock-version")
        .long("lock-version")
        .short('L')
        .action(ArgAction::Set)
        .value_name("X.Y.Z")
        .help(r#"Define a dev. kit version by a regex ("X.Y.Z" -> greater or equal | "=X.Y.Z" -> strictly equals)"#)
}

fn arg_main_func() -> Arg {
    Arg::new("main-func")
        .long("main-func")
        .short('M')
        .action(ArgAction::Set)
        .value_name("CLASS PATH")
        .help(r#"Path to `public static void main` entrypoint ("Main" as default). Expecting package format, like: "example.JavaClass"#)
}

#[cfg(test)]
mod tests {

    use super::subcommand;
    use crate::utils::ArgMatchesUtil;

    #[test]
    fn test() {
        let sub = subcommand();
        let args = [
            "new",
            "SomeName",
            "--description",
            "Let's think about a cool description...",
            "-A",
            r#"author1, author2, author3"#,
            "--path",
            "cool-path",
            "--vcs",
            "pijul",
            "-S",
            "project-name",
            "-O",
            "out-name",
            "-L",
            "=24.1.07",
            "--main-func",
            "innerpack.NotMain",
        ];
        let mtc = sub.get_matches_from(args);
        assert_eq!(mtc.force_get_one("name"), "SomeName");
        assert_eq!(
            mtc.force_get_one("description"),
            "Let's think about a cool description..."
        );
        let authors = mtc.force_get_one("authors");
        let authors: Vec<_> = authors.split(", ").collect();
        assert_eq!(authors, ["author1", "author2", "author3"]);
        assert_eq!(mtc.force_get_one("path"), "cool-path");
        assert_eq!(mtc.force_get_one("vcs"), "pijul");
        assert_eq!(mtc.force_get_one("source-path"), "project-name");
        assert_eq!(mtc.force_get_one("out-path"), "out-name");
        assert_eq!(mtc.force_get_one("lock-version"), "=24.1.07");
        assert_eq!(mtc.force_get_one("main-func"), "innerpack.NotMain");
    }
}
