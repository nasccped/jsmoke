//! # JSmoke Application
//!
//! This module provides the standard JSmoke features as traits.
//!
//! ## Traits and its functions
//!
//! By default, all traits contains only functions that receives the
//! object's self reference as parameter (`&self`) and returns a
//! personalized output type:
//! ```no_run
//! pub trait TraitExample {
//!     type Output;
//!     fn function_example(&self) -> Self::Output;
//! }
//! ```
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
//! ## The `&self` parameter
//!
//! The object used to run the trait's function isn't intended to
//! change over run. This explain why using `&self` instead of
//! `&mut self`.
//!
//! Async implementing is strongly being considered. The implicit
//! `&self`'s lifetime can lead to breaking changes in the future :^(
