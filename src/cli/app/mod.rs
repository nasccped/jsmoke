mod flags;
pub mod subcommands;

use clap::{
    Parser,
    builder::{Styles, styling::AnsiColor},
};
use flags::GlobalFlags;
use std::ops::Deref;
use subcommands::AppSubcommands;

/// Styles used along the app struct.
const STYLE: Styles = Styles::styled()
    .header(AnsiColor::BrightGreen.on_default())
    .usage(AnsiColor::BrightGreen.on_default())
    .error(AnsiColor::BrightRed.on_default())
    .literal(AnsiColor::Cyan.on_default())
    .placeholder(AnsiColor::Yellow.on_default());

/// The jsmoke cli app struct. Holds the subcommand variants + top level flags.
#[derive(Parser, Debug)]
#[command(name = "jsmk", author = "nasccped", version, about, styles = STYLE)]
#[command(
    after_help = "Use `jsmk help` for more details.",
    arg_required_else_help = false
)]
pub struct App {
    /// The subcommand being used.
    #[command(subcommand)]
    subcommand: AppSubcommands,

    /// The global flags being used.
    #[command(flatten)]
    flags: GlobalFlags,
}

impl App {
    /// Returns the subcommand being used.
    pub fn subcommand(&self) -> &AppSubcommands {
        &self.subcommand
    }
}

impl Deref for App {
    type Target = GlobalFlags;
    fn deref(&self) -> &Self::Target {
        &self.flags
    }
}
