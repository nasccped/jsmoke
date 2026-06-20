use std::borrow::Cow;

/// Allows to produce a verbose messages from a given item.
///
/// An item is required to implement [`Verbose`] to be displayed by the
/// [`super::notifier::Notifier::notify_verbose`] function.
pub trait Verbose {
    /// Returns the verbose message from the `self` item ([`None`] as default - no message).
    ///
    /// Note that the returned value is a [`Cow`], which means that the message can be a string
    /// slice/self inner field reference ([`Cow::Borrowed`]) or an owned [`String`] too (with
    /// [`Cow::Owned`]).
    fn get_verbose_message(&self) -> Option<Cow<'_, str>> {
        None
    }
}
