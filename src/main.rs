mod cli;
mod services;
mod shared_models;
mod utils;

use clap::Parser;
use cli::{App, AppParseFail};
use services::AppService;

fn main() {
    let output = match App::try_parse() {
        Ok(_) => todo!("impl app run"),
        Err(e) => AppService::handle_parse_fail(AppParseFail::from(e)),
    };
    AppService::exit_with_code(output);
}
