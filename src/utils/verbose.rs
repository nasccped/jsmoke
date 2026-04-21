//! # Verbose related module
//!
//! Produce and handle verbose messages with [`verbose_wrapper`] macro, [`VerboseWrapper`] type and
//! it's inner fields.

/// Generates a [`VerboseWrapper`] item over a set of `$format => $arg1, $arg2, ...`.
///
/// Note that this macro runs the [`VerboseWrapper`] private functions.
///
/// ## Examples
/// ```
/// # let message: VerboseWrapper;
/// // empty message
/// message = verbose_wrapper!();
/// assert_eq!(message.get_message(), None);
/// // multirow message
/// message = verbose_wrapper!(
///     "row one";
///     "row two";
/// );
/// assert_eq!(message.get_message(), Some("row one\nrow two"));
/// // args message
/// message = verbose_wrapper!(
///     "some {}!" => "message";
/// );
/// assert_eq!(message.get_message(), Some("some message!"));
/// ```
#[macro_export]
macro_rules! verbose_wrapper {
    {
        $(
            $form:literal $(=> $($arg:expr),*)?
        );* $(;)?
    } => {{
        let v: Vec<String> = vec![
            $(
                format!($form $(, $($arg),* )?)
            ),*
        ];
        VerboseWrapper(__VWrapper::from(v))
    }};
}

/// Verbose trait. Allows to produce a [`VerboseWrapper`] struct over the `self` item.
pub trait Verbose {
    /// Returns the [`VerboseWrapper`] from the `self` item reference.
    fn as_verbose(&self) -> VerboseWrapper;
}

/// Special type for verbose message wrapping. It handle message storing by [`verbose_wrapper`]
/// macro and it's private functions.
pub struct VerboseWrapper(__VWrapper);

impl VerboseWrapper {
    /// Push a new element into a new line.
    pub fn pushln<S: AsRef<str>>(&mut self, item: S) -> &Self {
        self.0.push(item.as_ref(), true);
        self
    }

    /// Push a new element without a new line.
    pub fn push<S: AsRef<str>>(&mut self, item: S) -> &Self {
        self.0.push(item.as_ref(), false);
        self
    }

    /// Returns the private field inner message.
    pub fn get_message(&self) -> Option<&str> {
        self.0.message()
    }
}

/// Private enum for [`VerboseWrapper`] inner data.
enum __VWrapper {
    /// When there's some message to print.
    Some(String),
    /// When there's no message to print.
    None,
}

impl From<Vec<String>> for __VWrapper {
    fn from(value: Vec<String>) -> Self {
        if value.is_empty() {
            Self::none()
        } else {
            Self::some_from(value.join("\n"))
        }
    }
}

impl __VWrapper {
    /// Creates a new [`__VWrapper::None`] variant item.
    fn none() -> Self {
        Self::None
    }

    /// Creates a new empty [`__VWrapper::Some`] variant item from the given string message.
    fn some_from(message: String) -> Self {
        Self::Some(message)
    }

    /// Pushes a new message sentence to the [`__VWrapper`] self item. Checks if a newline should
    /// be added between the previous/current sentence.
    fn push(&mut self, message: &str, new_line: bool) {
        match self {
            Self::Some(s) if new_line => {
                (*s).push('\n');
                (*s).push_str(message);
            }
            Self::Some(s) => {
                (*s).push_str(message);
            }
            _ => *self = Self::some_from(message.into()),
        }
    }

    /// Returnst the [`__VWrapper`] inner message (if exists).
    fn message(&self) -> Option<&str> {
        match self {
            Self::Some(m) => Some(m.as_str()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let message = verbose_wrapper!();
        assert_eq!(message.get_message(), None);
    }

    #[test]
    fn single() {
        let message = verbose_wrapper!("my message");
        assert_eq!(message.get_message(), Some("my message"));
    }

    #[test]
    fn compound() {
        let message = verbose_wrapper!(
            "my message";
            "is cool!";
        );
        assert_eq!(message.get_message(), Some("my message\nis cool!"));
    }

    #[test]
    fn args() {
        let message = verbose_wrapper!(
            "my message {} {}!" => "is", "cool";
            "what {} {} think?" => "do", "you";
        );
        assert_eq!(
            message.get_message(),
            Some("my message is cool!\nwhat do you think?")
        );
    }

    #[test]
    fn doc_testing() {
        let mut message = verbose_wrapper!();
        assert_eq!(message.get_message(), None);
        message = verbose_wrapper!(
            "row one";
            "row two";
        );
        assert_eq!(message.get_message(), Some("row one\nrow two"));
        message = verbose_wrapper!(
            "some {}!" => "message";
        );
        assert_eq!(message.get_message(), Some("some message!"));
    }

    #[test]
    fn pushing() {
        let mut message = verbose_wrapper!();
        message.pushln("first row");
        assert_eq!(message.get_message(), Some("first row"));
        message.pushln("second row");
        assert_eq!(message.get_message(), Some("first row\nsecond row"));
        message.push(" with item!");
        assert_eq!(
            message.get_message(),
            Some("first row\nsecond row with item!")
        );
    }
}
