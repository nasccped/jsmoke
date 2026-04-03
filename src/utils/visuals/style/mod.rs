//! # Visual styling module
//!
//! Applies styling to the string (and similars) based on the coloring impl block (apply color
//! on context instead of hardcode).
mod command;
mod quotes;
mod suggestion;
mod ticks;

pub use command::CommandStyle;
pub use suggestion::SuggestionStyle;
