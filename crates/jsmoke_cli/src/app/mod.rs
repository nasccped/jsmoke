mod style;

use clap::Parser;

/// The JSmoke cli app.
#[derive(Parser)]
#[command(
    name = "jsmk",
    bin_name = "jsmk",
    author = "nasccped",
    version = env!("CARGO_PKG_VERSION"),
    about = "A simple project manager for simple java apps",
    styles = style::APP_STYLE
)]
pub struct JsmkApp {
    /// Enable verbose (detailed info printing) during process.
    #[arg(long)]
    verbose: bool,
}
