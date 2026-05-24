use std::fmt::Display;

/// Add warning notifier to the item. Works similar to [`super::NotifyFailure`] but for warning
/// cases.
///
/// Items are required to also implement [`Display`].
pub trait WarningNotifiable: Display {}
