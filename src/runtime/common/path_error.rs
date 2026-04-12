use crate::utils::Verbose;
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
    fn print_verbose(&self) {
        match self {
            Self::Current => {
                // FIXME: use rust_code styling for printing bellow.
                eprintln!(
                    "This happens when Rust's {} function fails.\n",
                    CURRENT_PATH_FUNCTION
                );
                eprintln!("The possible causes (according to function's doc) are:");
                CURRENT_DIR_FUNCTION_ERR_CAUSES
                    .into_iter()
                    .for_each(|cause| {
                        eprintln!("- {}", cause);
                    });
            }
        }
    }
}
