//! # Verbose module
//!
//! Provides the [`PrintVerbose`] trait and its doc.

/// Trait for item's verbose printing.
pub trait PrintVerbose {
    /// Print verbose for the item reference. Note that this function
    /// will print the content independently if `--verbose` flag is
    /// enabled or not. Instead, consider checking `--verbose` flag
    /// first, and then, print if necessary. Example:
    /// ```compile_fail
    /// let cli_app = app_gen();
    /// let target_subcmd = cli_app.subcmd;
    /// if cli_app.is_verbose {
    ///     target_subcmd.print_verbose();
    /// }
    /// ```
    fn print_verbose(&self);
}
