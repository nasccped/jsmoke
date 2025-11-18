//! # Verbose module
//!
//! Provides the [`Verbose`] trait and its doc.

/// Verbose item related features.
pub trait Verbose {
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
    ///
    /// # Default behavior
    ///
    /// Not all types require being verbose. So, a default function
    /// is implemented (where it prints the value returned  from
    /// [`Verbose::get_verbose`], function).
    ///
    /// If the printed message must be different from the message
    /// returned by the [`Verbose::get_verbose`], consider
    /// overriding this function.
    ///
    /// Note that this function will print a new line at the end of
    /// the message only if the message isn't an empty string.
    fn print_verbose(&self) {
        match self.get_verbose().as_str() {
            "" => {}
            message => println!("\n{}\n", message),
        }
    }

    /// Returns the verbose message of the `self` item. The returned
    /// type should be an owned String (avoid lifetime headaches).
    ///
    /// If there's no verbose message, an empty String will be
    /// returned (default implementation).
    fn get_verbose(&self) -> String {
        "".into()
    }
}
