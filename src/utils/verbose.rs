/// Verbose trait. Allows to hold/show verbose data (used when `verbose` flag is enabled).
pub trait Verbose {
    /// Prints the verbose data based on the `self` item reference.
    fn print_verbose(&self);
}
