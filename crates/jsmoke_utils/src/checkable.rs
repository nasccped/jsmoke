//! # Checkable module
//!
//! Provides the [`Checkable`] trait.

/// Allows item checking. Works similar to [`TryFrom`] or
/// [`std::str::FromStr`] traits, but instead of explicitly returning
/// a [`Result`] type, it returns a personalized output type. Useful
/// when trying to check types that can emit warnings.
pub trait Checkable {
    /// Type to be returned (strongly recommend
    /// [`Option<GenericWarning>`] or [`Result<_, FatalErrorType>`]).
    type Output;

    /// Checks the self item, returning the [`Checkable::Output`] at
    /// the end.
    fn check(&self) -> Self::Output;
}
