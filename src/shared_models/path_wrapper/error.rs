use std::path::PathBuf;

/// Possible errors for [`super::PathWrapper`] parsing.
#[derive(thiserror::Error, Debug)]
pub enum PathWrapperError {
    /// When the parents of the final item doesn't exists.
    #[error("the parent path to the final item doesn't exists ({})", .0.to_string_lossy())]
    ParentDoesNotExists(PathBuf),
    /// When final item path already exists.
    #[error("the final path already exists ({})", .0.to_string_lossy())]
    PathAlreadyExists(PathBuf),
    /// When the provided path isn't valid.
    #[error("the provided string isn't a valid path ({})", .0)]
    InvalidPath(String),
}
