use clap::builder::{Styles, styling};

pub const APP_NAME: &str = "jsmoke";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_ABOUT: &str = "Simple project manager for simple java apps";
pub const APP_STYLE: Styles = Styles::styled()
    .header(styling::AnsiColor::BrightRed.on_default())
    .usage(styling::AnsiColor::BrightRed.on_default())
    .literal(styling::AnsiColor::Cyan.on_default())
    .placeholder(styling::AnsiColor::Cyan.on_default());
pub const APP_AUTHOR: &str = "nasccped";
