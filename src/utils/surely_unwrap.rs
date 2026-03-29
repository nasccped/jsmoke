use std::{
    any::type_name,
    fmt::{Debug, Display},
};

/// This trait unwraps types that are surely [`Ok`] or [`Some`] variant.
/// Works the same as [`Option::unwrap`] or [`Option::expect`], but with an explicit func name.
///
/// Anyone that panics at this knows that wasn't a missed [`Option::unwrap`] call, but a value that
/// surely must return something...
pub trait SurelyUnwrap<T: Debug> {
    /// Surely unwraps an [`Option`] or [`Result`] value.
    ///
    /// When failing, automatically panics + prints the expected type name and the `self` debug
    /// display.
    fn surely_unwrap(self) -> T;
}

/// Representation of a _'displayable'_ generic type.
enum ExpectedValue {
    /// When the expeceted value is [`Some`].
    SomeVariant(&'static str),
    /// When the expeceted value is [`Ok`].
    OkVariant(&'static str),
}

impl Display for ExpectedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExpectedValue::SomeVariant(t) => format!("`Some({})`", t),
                ExpectedValue::OkVariant(t) => format!("`Ok({})`", t),
            }
        )
    }
}

/// Private function. Prints the trait panic + value debug info.
fn panic_action<U: Debug>(expected: ExpectedValue, value: U) -> ! {
    panic!(
        "[PANIC] surely unwrap fails. expecting {}, but received {:?}",
        expected, value
    );
}

impl<T: Debug> SurelyUnwrap<T> for Option<T> {
    fn surely_unwrap(self) -> T {
        self.unwrap_or_else(|| {
            panic_action(
                ExpectedValue::SomeVariant(type_name::<T>()),
                None as Option<T>,
            )
        })
    }
}

impl<T: Debug, E: Debug> SurelyUnwrap<T> for Result<T, E> {
    fn surely_unwrap(self) -> T {
        // use matching instead of `unwrap_or_else` (move constraint)
        match self {
            Ok(v) => v,
            e => panic_action(ExpectedValue::OkVariant(type_name::<T>()), e),
        }
    }
}
