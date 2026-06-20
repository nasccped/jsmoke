use clap::{
    Parser, Subcommand,
    builder::{Styles, styling::AnsiColor},
};

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

/// All available subcommand.
#[derive(Subcommand, Debug)]
pub enum AppSubcommands {
    /// Does 'a' related things.
    AStuff,

    /// Does 'b' related things.
    BStuff,
}

/// All available global flags.
#[derive(Parser, Debug)]
struct GlobalFlags {
    /// If the operation should be forced.
    #[arg(long)]
    force: bool,
}
