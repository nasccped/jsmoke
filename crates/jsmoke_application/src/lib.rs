//! [`Result<T, U>`]: Result<T, U>
//! [`Option<T>`]: Option<T>
//! [`Result`]: Result
//! [`Err`]: Err
//! [`std::error::Error`]: std::error::Error
//! [`Box<T: Error`]: Box<T: Error>
//! [`Result<_, Box<dyn Error>>`]: Result<_, Box<dyn Error>>
#![doc = include_str!("../README.md")]

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
