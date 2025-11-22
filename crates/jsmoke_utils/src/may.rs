//! # May module
//!
//! This module provides the `May`s traits.

/// May build the `Self` from a `T` value.
///
/// Works similar to a [`TryFrom`] trait, but instead of returning a
/// [`Result<Ok, Err>`], it returns an [`Option<Self>`] over the `T`
/// value.
///
/// It's useful when trying to detect an error of a complex type that
/// doesn't provide a [`Ok`] variant.
pub trait MayFrom<T>: Sized {
    /// May converts the `T` value into a `Self` one, returning
    /// [`None`] when fails.
    fn may_from(value: T) -> Option<Self>;
}

/// May convert `Self` into a `T` value.
///
/// Does the reverse of [`MayFrom`] trait. Consider implementing that
/// one since any type that implements [`MayFrom`] will automatically
/// implements the [`TryInto`] trait (default function).
pub trait MayInto<T: MayFrom<Self>>: Sized {
    /// May converts the `Self` value into a `T` one, returning
    /// [`None`] when it fails.
    fn may_into(value: Self) -> Option<T> {
        T::may_from(value)
    }
}
