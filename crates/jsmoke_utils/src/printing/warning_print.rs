//! # Warning printing module
//!
//! Provides warning printing reserved utilities (majorly traits).
use crate::visuals::tags::TagKind;

/// Tag used when printing an warning.
const WARN_TAG: TagKind = TagKind::WarningKind;

/// Trait for pretty warning printing.
///
/// This trait can only be applied to types that also implements
/// [`std::error::Error`] trait.
///
/// Even not being an error at all, this is a short-hand for
/// warning features using the [`thiserror::Error`] proc-macro. This
/// trait constraint allows to simply implements the [`WarningPrint`]
/// trait using a `proc-macro` like syntax provided by the
/// [`jsmoke_macros`] crate:
///
/// ```compile_fail
/// // use this:
/// #[derive(WarningPrint)]
/// enum WarningVariants {
///     // ...
/// }
/// // instead of this:
/// impl WarningPrint for WarningVariants {
///     // ...
/// }
/// ```
pub trait WarningPrint: std::error::Error {
    fn print_warning(&self) {
        println!("{}{}", WARN_TAG, self);
    }
}
