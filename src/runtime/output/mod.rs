//! # Runtime's output module
//!
//! Provides outputs types, traits and wrappers for easy Ok/Err handling.
mod runtime;
mod wrappers;

pub use runtime::{IntoExitCode, OutputNotify, RuntimeOutput};
pub use wrappers::{FailureConstraint, IntoErr, IntoOk, SuccessConstraint};
