//! # Jsmoke Regex module
//!
//! This module provides the [`regex::Regex`] wrapper, its
//! [`TryFrom`] / [`TryInto`] convertions and simple shorthand
//! features.
//!
//! Some impl and data types are private, but the main content
//! ([`RegexBuilder`] and [`RegexWrapper`]) are public. Read their
//! docs.
mod how_much_of_pattern;
mod regex_builder;
mod regex_wrapper;

pub use regex_builder::RegexBuilder;
pub use regex_wrapper::RegexWrapper;
