use std::fmt::Display;

/// Custom output for [`super::Styler`] trait.
pub enum StylerOutput<T: Display> {
    /// Means a single item.
    Item(T),

    /// Means a collection of items (separated by newlines).
    Items(Vec<T>),
}

impl<T: Display> Display for StylerOutput<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Self::Item(item) => item.to_string(),
            Self::Items(items) => items
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        };
        write!(f, "{}", content)
    }
}
