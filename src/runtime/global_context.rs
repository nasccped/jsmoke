use super::{GlobalPrinter, common::PathError};
use std::{path::PathBuf, rc::Rc};

/// Program's global fields used across runtime.
pub struct GlobalContext {
    /// Global CLI triggers (accessible through flags).
    flags: FlagTriggers,
    /// Current user path.
    curpath: Result<PathBuf, PathError>,
    /// Printer helper.
    printer: Rc<GlobalPrinter>,
}

impl GlobalContext {
    /// Creates a new [`GlobalContext`] item with default fields.
    pub fn new() -> Self {
        let flags = FlagTriggers::default();
        let curpath = std::env::current_dir().map_err(|_| PathError::Current);
        let printer = Rc::new(GlobalPrinter::default());
        Self {
            flags,
            curpath,
            printer,
        }
    }

    /// Set a force value to [`FlagTriggers`].
    pub fn with_force(&mut self, force: bool) -> Self {
        self.flags.force = force;
        *self
    }

    /// Set a verbose value to [`FlagTriggers`].
    pub fn with_verbose(&mut self, verbose: bool) -> Self {
        self.flags.verbose = verbose;
        *self
    }

    /// If the `force` trigger is enabled.
    pub fn is_forced(&self) -> bool {
        self.flags.force
    }

    /// If the `verbose` trigger is enabled.
    pub fn is_verbose(&self) -> bool {
        self.flags.verbose
    }

    /// Returns the inner printer reference.
    pub fn get_printer(&self) -> Rc<GlobalPrinter> {
        self.printer
    }
}

/// CLI flag triggers.
#[derive(Default)]
struct FlagTriggers {
    /// Force flag (`--force`).
    force: bool,
    /// Verbose flag (`--verbose`).
    verbose: bool,
}
