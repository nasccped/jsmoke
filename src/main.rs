mod cli;
mod exit;
mod utils;

use clap::Parser;
use std::process::ExitCode;
use utils::printer::Printer;

fn main() {
    let app = cli::App::parse();
    let mut printer = Printer::default();
    let green = |x: &str| format!("\x1b[92m{}\x1b[0m", x);
    let red = |x: &str| format!("\x1b[91m{}\x1b[0m", x);
    let yes_no = |condition: bool| if condition { green("yes") } else { red("no") };
    printer.println("Let's print content...");
    printer.empty_line();
    printer.print("subcommand called: ");
    printer.println(yes_no(app.subcommand.is_some()));
    printer.print("using force: ");
    printer.println(yes_no(app.force));
    printer.print("using verbose: ");
    printer.println(yes_no(app.verbose));
    printer.empty_line();
    if false {
        printer.use_stderr();
        printer.println("This block disable the clippy arguing");
        exit::with_code(ExitCode::FAILURE);
    } else {
        printer.println("Leaving with no errors");
    }
}
