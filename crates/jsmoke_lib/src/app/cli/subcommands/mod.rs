//! # `subcommands` module
//!
//! Store app's subcommands and add they in [`super::App::generate`].
//!
//! Each submodule of the current module ([`build`], [`init`],
//! [`new`], `...`) is responsible for its own subcommand construct.
//!
//! The submodules can have one or more functions, but only one
//! function (`subcommand` always) should be public exported.
//!
//! ## Example
//!
//! All subcommands module should looks like this:
//! ```no_run
//! //! `new.rs` file
//! use clap::{Command, Arg};
//!
//! pub fn subcommand() -> Command {
//!     todo!("Implement `new` subcommand + extra pieces...")
//! }
//!
//! // private functions
//! fn extra_piece_1() -> Arg {
//!     todo!("piece 1 logic...")
//! }
//!
//! fn extra_piece_2() -> Arg {
//!     todo!("piece 2 logic...")
//! }
//!
//! //...
//! ```
//!
//! - [`new::subcommand`] will return the `new` subcommand
//! - [`init::subcommand`] will return the `init` subcommand
//! - `...`

mod new;

pub use new::subcommand as new;
