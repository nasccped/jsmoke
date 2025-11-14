use clap::builder::{Styles, styling};

pub const APP_STYLE: Styles = Styles::styled()
    .header(styling::AnsiColor::BrightGreen.on_default().bold())
    .usage(styling::AnsiColor::BrightGreen.on_default().bold())
    .literal(styling::AnsiColor::BrightCyan.on_default().bold())
    .placeholder(styling::AnsiColor::BrightCyan.on_default());
