use crate::utils::{
    notifiers::{NotifyFailure, NotifySuccess, NotifyWarning},
    verbose::Verbose,
};
use std::fmt::Display;

/// A global struct for content printing. Handle all the notify and verbose stuff by this item.
///
/// Note that this struct uses a `stderr` field to decide where to print the content. Don't forget
/// to change it before printing if necessary!
#[derive(Default)]
pub struct GlobalPrinter {
    /// Content should be print to stderr?
    stderr: bool,
}

impl GlobalPrinter {
    /// Set a new value to `stderr` field + returns the `self` ref.
    pub fn set_stderr(&mut self, value: bool) -> &Self {
        self.stderr = value;
        self
    }

    /// Prints the fail message based on `fail` item.
    ///
    /// **Note** that this function doesn't decides where to print (stderr/stdout). Use
    /// [`GlobalPrinter::set_stderr`] before printing if necessary.
    pub fn print_failure<F: NotifyFailure>(&self, fail: F) {
        self.private_print(fail.get_fail_message());
    }

    /// Prints the success message based on `success` item.
    pub fn print_success<S: NotifySuccess>(&self, success: S) {
        self.private_print(success.get_success_message());
    }

    /// Prints the warning message based on `warning` item.
    pub fn print_warning<W: NotifyWarning>(&self, warning: W) {
        self.private_print(warning.get_warning_message());
    }

    /// Prints the verbose message from the given `item` (if it exists).
    ///
    /// **Note** that this function doesn't decides where to print (stderr/stdout). Use
    /// [`GlobalPrinter::set_stderr`] before printing if necessary.
    pub fn print_verbose<V: Verbose>(&self, item: V) {
        if let Some(message) = item.as_verbose().get_message() {
            self.private_print("\n");
            self.private_print(message);
        }
    }

    /// Just wrapper for [`GlobalPrinter::private_print`]. Necessary since that function is
    /// private.
    ///
    /// **Note** that this function doesn't decides where to print (stderr/stdout). Use
    /// [`GlobalPrinter::set_stderr`] before printing if necessary.
    pub fn simple_print<T: Display>(&self, item: T) {
        self.private_print(item);
    }

    /// Print into target based on `stderr` field.
    fn private_print<T: Display>(&self, item: T) {
        if self.stderr {
            eprintln!("{}", item);
        } else {
            println!("{}", item);
        }
    }
}
