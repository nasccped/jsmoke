//! # Error printing module
//!
//! Provides error printing reserved utilities (majorly traits).
use crate::visuals::tags::ERROR_TAG;

/// Trait for pretty error printing.
///
/// This trait is expected to be applied to any type that also
/// implements the [`std::error::Error`] and the
/// [`std::fmt::Display`] traits (since both traits are auto impl if
/// the given type have [`derive`] with [`thiserror::Error`]).
pub trait ErrorPrint: std::error::Error {
    fn print_error(&self) {
        println!("{}{}", ERROR_TAG, self);
    }
}
