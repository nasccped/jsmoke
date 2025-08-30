//! # `jsmoke` lib crate
//!
//! This crate is intended to provide usefull jsmoke's logic +
//! features. Instead of mixing implementation and declaration, just
//! separate them into small pieces and allow code reuse to be ran
//! into a closed environment.
//!
//! ## Extra info
//!
//! This crate (lib) can be separated into three different
//! submodules:
//!
//! - [`app`]: provides app features and logic implementation
//! - [`config`]: provides app's config (const values)
//! - [`utils`]: types and traits utilities that should be used/
//!   implemented by the `App` core object.
//!
//! [`config`] and [`utils`] are private, since user needs only the
//! `App` instance + run logic. All the _"hands on"_ stuff is handled
//! internally by this lib.
pub mod app;

mod config;
mod utils;
