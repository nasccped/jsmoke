mod cli;
mod models;
mod services;
mod shared_models;
mod utils;

use clap::Parser;
use cli::{App, AppParseFail};
use services::AppService;

fn main() {
    let output = match App::try_parse() {
        Ok(app_state) => match AppService::try_from(&app_state) {
            Ok(_) => {
                todo!("Parse success. Behavior still in progress...");
            }
            Err(err) => {
                err.self_notify();
                err.into()
            }
        },
        Err(e) => AppService::handle_parse_fail(AppParseFail::from(e)),
    };
    AppService::exit_with_code(output);
}
