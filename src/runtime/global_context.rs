use super::{GlobalPrinter, common::PathError};
use std::{
    cell::{Ref, RefCell, RefMut},
    path::PathBuf,
    rc::Rc,
};

/// Program's global fields used across runtime.
pub struct GlobalContext {
    /// Global CLI triggers (accessible through flags).
    flags: FlagTriggers,
    /// Current user path.
    curpath: Result<PathBuf, PathError>,
    /// Printer helper.
    printer: Rc<RefCell<GlobalPrinter>>,
}

impl Default for GlobalContext {
    fn default() -> Self {
        let flags = FlagTriggers::default();
        let curpath = std::env::current_dir().map_err(|_| PathError::Current);
        let printer = Rc::new(RefCell::new(GlobalPrinter::default()));
        Self {
            flags,
            curpath,
            printer,
        }
    }
}

impl GlobalContext {
    /// Set a force value to [`FlagTriggers`].
    pub fn with_force(&mut self, force: bool) -> &mut Self {
        self.flags.force = force;
        self
    }

    /// Set a verbose value to [`FlagTriggers`].
    pub fn with_verbose(&mut self, verbose: bool) -> &mut Self {
        self.flags.verbose = verbose;
        self
    }

    /// If the `force` trigger is enabled.
    pub fn is_forced(&self) -> bool {
        self.flags.force
    }

    /// If the `verbose` trigger is enabled.
    pub fn is_verbose(&self) -> bool {
        self.flags.verbose
    }

    /// Returns the inner printer reference (**imutable**).
    pub fn get_printer(&self) -> Ref<'_, GlobalPrinter> {
        self.printer.borrow()
    }

    /// Returns the inner printer reference (**mutable**).
    pub fn get_mut_printer(&mut self) -> RefMut<'_, GlobalPrinter> {
        self.printer.borrow_mut()
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
