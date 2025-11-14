/// Exit the current program with the provided code value:
/// - ok/warning as `0`
/// - internal error as `N` when `N` isn't equals than `0`
///
/// No effect when exiting with `0` (means the same as reaching the
/// end of the code).
fn exit_with_code(code: i32) -> ! {
    std::process::exit(code);
}

fn main() {
    // unused var
    let _app = jsmoke_cli::parse();
    exit_with_code(0);
}
