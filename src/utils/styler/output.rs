use super::ListStyle;
use colored::Colorize;
use std::fmt::Display;

/// Custom output for [`super::Styler`] trait.
pub enum StylerOutput<T: Display> {
    /// Means a single item.
    Item(T),

    /// Means a collection of items (separated by newlines).
    Items(Vec<T>),
}

impl From<ListStyle> for StylerOutput<String> {
    fn from(value: ListStyle) -> Self {
        let is_ordered = value.is_ordered();
        let padding = value.padding();
        let items = value.consume();
        let indicators = (1..=items.len()).map(|ind| {
            if is_ordered {
                format!("{}.", ind).cyan()
            } else {
                "*".cyan()
            }
        });
        let items = items
            .into_iter()
            .zip(indicators)
            .map(|(item, ind)| format!("{}{} {}", " ".repeat(padding), ind, item))
            .collect();
        Self::Items(items)
    }
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
