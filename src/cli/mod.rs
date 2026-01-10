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
    /// Turn on the verbosed mode.
    #[arg(long)]
    verbose: bool,
}

#[cfg(test)]
mod test {
    use super::{App, Parser};

    const EMPTY: [&str; 1] = ["jsmk"];
    const FORCE: [&str; 2] = ["jsmk", "--force"];
    const VERBOSE: [&str; 2] = ["jsmk", "--verbose"];

    #[test]
    fn no_subcommand_and_no_force() {
        assert!(App::parse_from(EMPTY).subcommand.is_none());
        assert!(!App::parse_from(EMPTY).force);
    }

    #[test]
    fn force_trigger() {
        assert!(App::parse_from(FORCE).force);
    }

    #[test]
    fn verbose_trigger() {
        assert!(App::parse_from(VERBOSE).verbose);
    }
}
