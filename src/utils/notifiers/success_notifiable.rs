use std::fmt::Display;

/// Add success notifier to the item. Works similar to [`super::NotifyFailure`] but for success
/// cases.
///
/// Items are required to also implement [`Display`].
pub trait SuccessNotifiable: Display {}
