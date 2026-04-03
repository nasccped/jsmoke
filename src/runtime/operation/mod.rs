//! # Operation module
//!
//! Provides jsmoke standard operations (based on subcommands).
mod new;

use super::RuntimeOutput;
pub use new::NewOperation;

/// General trait for operation types. Provides item generation + running.
pub trait OperationTrait {
    /// Runs the operation type.
    fn run(self, force: bool, verbose: bool) -> RuntimeOutput;
}
