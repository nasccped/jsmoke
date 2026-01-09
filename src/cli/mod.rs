//! # CLI
//!
//! Provides the cli main features (constructor and components).
use clap::Parser;
pub mod subcommands;

/// The jsmoke cli app struct. Holds the subcommand variants + top level flags.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct App {
    /// The subcommand to be executed.
    #[command(subcommand)]
    subcommand: Option<subcommands::Subcommand>,
    /// Force the called action.
    #[arg(long)]
    force: bool,
}

#[cfg(test)]
mod test {
    use super::{App, Parser, subcommands::Subcommand};

    const EMPTY: [&str; 1] = ["jsmk"];
    const FORCE: [&str; 2] = ["jsmk", "--force"];

    #[test]
    fn no_subcommand_and_no_force() {
        assert!(App::parse_from(EMPTY).subcommand.is_none());
        assert!(!App::parse_from(EMPTY).force);
    }

    #[test]
    fn force_trigger() {
        dbg!(App::parse_from(FORCE));
        assert!(App::parse_from(FORCE).force);
    }

    #[test]
    fn expected_subcommands() {
        // input as vec (add alias soon).
        let samples = [
            (vec!["build"], Subcommand::Build),
            (vec!["clean"], Subcommand::Clean),
            (vec!["init"], Subcommand::Init),
            (vec!["new"], Subcommand::New),
            (vec!["run"], Subcommand::Run),
        ];
        for (inps, exps) in samples {
            assert!(inps.iter().all(|cmd| {
                App::parse_from(["jsmk", cmd])
                    .subcommand
                    .is_some_and(|cmd| cmd == exps)
            }));
        }
    }
}
