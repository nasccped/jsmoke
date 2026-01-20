/// Verbose trait. Allows to hold/show verbose data (used when `verbose` flag is enabled).
pub trait Verbose {
    /// An [`super::printer::Printer`] like type to allow smart printing (stdout/stderr).
    type LocalPrinter<'a>;
    /// Prints the verbose data based on the `self` item reference.
    fn print_verbose(&self, printer: Self::LocalPrinter<'_>);
}
