use clap::Parser;

/// Global flags used across the runtime.
#[derive(Parser, Debug)]
pub struct GlobalFlags {
    /// If the operation should be forced.
    #[arg(long, global = true)]
    force: bool,

    /// If the operation should be verbose.
    #[arg(long, global = true)]
    verbose: bool,
}

impl GlobalFlags {
    /// If the operation is being verbose.
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    /// If the operation is being forced.
    pub fn is_forced(&self) -> bool {
        self.force
    }
}
