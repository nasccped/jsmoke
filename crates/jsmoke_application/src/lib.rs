//! # JSmoke Application
//!
//! This module provides the standard JSmoke features as traits.
//!
//! ## Traits and its functions
//!
//! By default, all traits contains only functions that receives the
//! object's self reference as parameter (`&self`) and returns a
//! personalized output type:
//!
//! ```rust
//! pub trait TraitExample {
//!     type Output;
//!     fn function_example(&self) -> Self::Output;
//! }
//! ```
//!
//! ## The `&self` parameter
//!
//! The object used to run the trait's function isn't intended to
//! change over run. This explain why using `&self` instead of
//! `&mut self`.
//!
//! Asynchronous implementing is strongly being considered. The
//! implicit `&self`'s lifetime can lead to breaking changes in the
//! future :^(
//!
//! ## Output type
//!
//! The personalized output can turn error handling more flexible
//! instead of panicking the entire program. `Output` is preferable
//! to be a [`Result<T, U>`] or [`Option<T>`].
//!
//! When `Output` is a [`Result`] type, it's recommended that the
//! [`Err`] variant implements the [`std::error::Error`] trait for a
//! better error handling (by standard types/funcs that only accepts
//! [`Box<T: Error>`] or something).
//!
//! When a single function returns different kind of errors, consider
//! using [`Result<_, Box<dyn Error>>`] instead. Since the [`Err`]
//! variant implements the [`std::error::Error`] trait, any error can
//! be used.

/// Create a new project using this trait. Note that `init` and `new`
/// subcommands can both implement it since they do the same job.
pub trait NewProject {
    /// Output type when creating a new project. [`Result<T, U>`]
    /// type is strongly recommended. Check [`crate`] documentation
    /// for more `Output` tips and details.
    type Output;

    /// Create a new project taking data from the object that
    /// implements this trait and returning a [`NewProject::Output`]
    /// value.
    fn new_project(&self) -> Self::Output;
}

/// Build project using this trait. Note that a [`RunProject`]
/// function's call can also call this trait's functions _(since
/// running requires a built project)_.
pub trait BuildProject {
    /// Output type when building a project. [`Result<T, U>`] type
    /// is strongly recommended. Check [`crate`] documentation for
    /// more `Output` tips and details.
    type Output;

    /// Build the project taking data from the object that implements
    /// this trait and return a [`BuildProject::Output`] value.
    fn build_project(&self) -> Self::Output;
}

/// Run projects using this trait. Note that calling this trait's
/// function can also call the [`BuildProject`]'s functions _(since
/// running requires a built project)_.
pub trait RunProject {
    /// Output type when running a project. [`Result<T, U>`] type
    /// is strongly recommended. Check [`crate`] documentation for
    /// more `Output` tips and details.
    type Output;

    /// Run the project's bytecode. If built files doesn't
    /// exists/isn't updated, [`BuildProject::build_project`] will
    /// probably be called.
    fn run_project(&self) -> Self::Output;
}

/// Clear built files (bytecodes/logs) from the project's directory.
pub trait ClearProject {
    /// Output type when clear is done. Since there's no important
    /// data to return, output type can be unit `()` or
    /// [`Option<dyn Error>`] (for privileges/not found kind of
    /// error).
    type Output;

    /// Self-explained.
    fn clear_project(&self) -> Self::Output;
}

/// Log project's build status (errors/warnings).
pub trait LogProject {
    /// Type to be returned when trying to log status.
    type Output;

    /// Show status log based on the object's inner fields.
    fn log_project(&self) -> Self::Output;
}
