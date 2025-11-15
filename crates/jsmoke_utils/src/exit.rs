//! # Exit Module
//!
//! Program abort utilities.

/// Wrapper for [`std::process::exit`] function.
///
/// Exit the current program with the provided code value:
/// - ok/warning as `0`
/// - internal error as `N` when `N` isn't equals than `0`
///
/// No effect when exiting with `0` (means the same as reaching the
/// end of the code).
pub fn exit_with_code(code: i32) -> ! {
    std::process::exit(code);
}
