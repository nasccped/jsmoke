//! # Visual styling module
//!
//! Applies styling to the string (and similars) based on the coloring impl block (apply color
//! on context instead of hardcode).
mod command;
mod dollar;
mod list;
mod number;
mod path;
mod quotes;
mod suggestion;
mod term;
mod ticks;
mod weak;

pub use command::CommandStyle;
pub use list::ItemList;
pub use number::NumberStyle;
pub use path::PathStyle;
pub use suggestion::SuggestionStyle;
pub use term::TermStyle;
pub use weak::WeakStyle;
