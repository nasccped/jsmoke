use crate::{
    utils::{
        verbose::{Verbose, VerboseWrapper},
        visuals::style::ItemList,
    },
    verbose_wrapper,
};
use thiserror::Error as ThisError;

/// Rust's [`std::env::current_dir`] function.
const CURRENT_PATH_FUNCTION: &str = "std::env::current_dir()";

/// Probably causes that leads [`std::env::current_dir`] function to returns an error.
const CURRENT_DIR_FUNCTION_ERR_CAUSES: [&str; 2] =
    ["current directory doesn't exists", "not enough privileges"];

/// Common errors when trying to load/generate a new [`std::path::Path`] kind value at runtime.
#[derive(ThisError, Debug)]
pub enum PathError {
    /// Current path couldn't be load.
    #[error("failed to get the current path")]
    Current,
}

impl Verbose for PathError {
    fn as_verbose(&self) -> VerboseWrapper {
        match self {
            Self::Current => {
                let mut vw = verbose_wrapper!(
                    "This happens when Rust's {} function fails." =>
                        CURRENT_PATH_FUNCTION;
                    "The possible causes (according to function's doc) are:";
                );
                CURRENT_DIR_FUNCTION_ERR_CAUSES
                    .into_iter()
                    .for_each(|cause| {
                        vw.pushln(cause.item_list_style());
                    });
                vw
            }
        }
    }
}
