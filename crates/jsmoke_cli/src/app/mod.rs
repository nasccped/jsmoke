//! # App module
//!
//! Provides the main CLI app implementing.
//!
//! All the implementing is done through the [`clap::Parser`]
//! proc-macro.

mod style;
pub mod subcommands;
use clap::Parser;
use subcommands::JsmkSubcommand;

/// The JSmoke cli app.
#[derive(Parser)]
#[command(
    name = "jsmk",
    bin_name = "jsmk",
    author = "nasccped",
    about = "A simple project manager for simple java apps",
    styles = style::APP_STYLE
)]
pub struct JsmkApp {
    /// Operation to perform.
    #[command(subcommand)]
    pub subcommand: Option<JsmkSubcommand>,

    /// Enable verbose (detailed info printing) during process.
    #[arg(long, global = true)]
    pub verbose: bool,
}
