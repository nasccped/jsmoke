//! # Operation module
//!
//! Provides jsmoke standard operations (based on subcommands).
mod new;

use super::RuntimeOutput;
use crate::runtime::Context;
pub use new::New;

/// Type alias for [`OperationTrait`] as dynamic [`Box`].
type BoxedOperation = Box<dyn OperationTrait + 'static>;

/// General trait for operation types. Provides item generation + running.
pub trait OperationTrait
where
    Self: 'static,
{
    /// Runs the operation type.
    fn run(&self, gctx: Context) -> RuntimeOutput;

    /// Converts the self operation into it's dyn boxed trait.
    fn into_boxed(self) -> BoxedOperation
    where
        Self: Sized,
    {
        Box::new(self) as BoxedOperation
    }
}
