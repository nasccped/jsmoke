//! # Common module
//!
//! Provides common utilities across the entire CLI implementing.

/// Enable verbose for the item that implements it.
///
/// Since not all items provides a `verbose` field as the main app
/// CLI, this trait allows an item to enable verbose even if not
/// having access to the app's verbose flag.
///
/// # Example
///
/// Consider that you have subcommand args to deal with:
///
/// ```rust
/// # use clap::Parser;
/// #[derive(Parser)]
/// pub struct FooSubcommand {
///     #[arg(long)]
///     baz: Option<String>,
///     bar: bool,
/// }
/// ```
///
/// This command can run in verbose or not, but the `verbose` flag is
/// global and outside of the `FooSubcommand` inner fields.
///
/// To solve this, we can implement the [`Verbose`]'s trait
/// which provides default functions (func. implement isn't required)
/// that can verbose handling a lot easier.
///
/// # Important
///
/// This trait operates with the [`IS_VERBOSE_MODE`] static
/// variable. It isn't thread safe guaranteed. Will be changed
/// (probably).
pub trait Verbose {
    /// **Turns on** the local verbosity trigger.
    fn enable_verbose(&self) {
        unsafe {
            IS_VERBOSE_MODE = true;
        }
    }
    /// **Returns** if the local verbosity trigger is **on or not**.
    fn is_verbose(&self) -> bool {
        unsafe { IS_VERBOSE_MODE }
    }
}

/// Local trigger to detect verbosity (`false` by default).
static mut IS_VERBOSE_MODE: bool = false;
