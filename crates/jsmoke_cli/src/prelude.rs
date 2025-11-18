//! # Prelude module
//!
//! Provides access to crate's inner data types.

pub mod app {
    //! # App module
    //!
    //! Provides the [`JsmkApp`] type access.
    pub use crate::app::JsmkApp;
}

pub mod subcommands {
    //! # Subcommands module
    //!
    //! Provides the [`subcommands`] mods/types access.
    #[allow(unused_imports)]
    use crate::app::subcommands;
    pub use crate::app::subcommands::JsmkSubcommand;
}
