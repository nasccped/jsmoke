//! # List module
//!
//! List related printing. Provides List components with easy building:
//!
//! - [`List`]: The list itself with private fields but good coverage for [`Vec`] features.
//! - [`Bullet`]: A Bullet sign. Tick being printed before the actual item.
//! - [`Item`]: Struct returned when calling [`List::into_iter`] which is majorly private but
//!   contains good pub interface (also [`Display`] impl).
use colored::{ColoredString, Colorize};
use std::fmt::Display;

/// An visual list type. It can hold any Item and bullet symbol that implements [`Colorize`] and
/// [`Display`] traits.
#[derive(Debug, Clone)]
pub struct List<T1: Colorize + Display, T2: Display> {
    /// The items itself.
    list: Vec<T1>,
    /// The color being used to colorize the items.
    item_color: fn(T1) -> ColoredString,
    /// The optional bullet.
    bullet: Option<Bullet<T2>>,
    /// The color being used to colorize the bullet.
    bullet_color: fn(&'_ str) -> ColoredString,
}

impl<T1: Colorize + Display, T2: Display> IntoIterator for List<T1, T2> {
    type Item = Item;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let bullet = self.bullet;
        self.list
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let inner_bullet = bullet.as_ref().map(|b| match b {
                    Bullet::Numeric(x) => format!("{}", x + i),
                    Bullet::NumericDot(x) => format!("{}.", x + i),
                    Bullet::NumericParen(x) => format!("{})", x + i),
                    Bullet::Symbol(s) => format!("{}", s),
                });
                Item {
                    bullet: inner_bullet.map(|b| (self.bullet_color)(b.as_str())),
                    item: (self.item_color)(item),
                }
            })
            .collect::<Vec<Item>>()
            .into_iter()
    }
}

/// Workaround for [`str`] slice convertion to [`ColoredString`] since [`String`] doesn't
/// implements [`Colorize`].
fn colored_string_from_str(s: &str) -> ColoredString {
    String::from(s).into()
}

impl<T1, T2> List<T1, T2>
where
    T1: Colorize + Display + Into<ColoredString>,
    T2: Colorize + Display + Into<ColoredString>,
    ColoredString: From<T1> + From<T2>,
{
    /// Creates a new [`List`] item with an empty list, no colors/bullet symbol. Use other
    /// functions to interact with inner data.
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            item_color: ColoredString::from,
            bullet: None,
            bullet_color: colored_string_from_str,
        }
    }

    /// If the list doesn't contains any item.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Returns the length of the list (how many items it contains).
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Returns a reference to the inner bullet field.
    pub fn get_bullet(&self) -> Option<&Bullet<T2>> {
        self.bullet.as_ref()
    }

    /// Sets a new color function to the list's items. Use [`Self::without_color`] to disable it.
    pub fn with_color(&mut self, color: fn(T1) -> ColoredString) -> &mut Self {
        self.item_color = color;
        self
    }

    /// Removes the color function from the self struct.
    pub fn without_color(&mut self) -> &mut Self {
        self.item_color = ColoredString::from;
        self
    }

    /// Extends the inner list with the provided items. Note that this function doesn't clear the
    /// list. If you want so, use [`Self::without_items`] instead.
    pub fn with_items<In: Into<T1>, It: IntoIterator<Item = In>>(
        &mut self,
        items: It,
    ) -> &mut Self {
        self.list.extend(items.into_iter().map(|item| item.into()));
        self
    }

    /// Clear all the items.
    pub fn without_items(&mut self) -> &mut Self {
        self.list.clear();
        self
    }

    /// Sets a new bullet for this list. Use [`Self::without_bullet`] to undo.
    pub fn with_bullet(&mut self, bullet: Bullet<T2>) -> &mut Self {
        self.bullet = Some(bullet);
        self
    }

    /// Clear the bullet symbol from the current list.
    pub fn without_bullet(&mut self) -> &mut Self {
        self.bullet = None;
        self
    }

    /// Sets a new bullet coloring function. Use [`Self::without_bullet_color`] to undo.
    pub fn with_bullet_color(&mut self, color: fn(&'_ str) -> ColoredString) -> &mut Self {
        self.bullet_color = color;
        self
    }

    /// Clears the coloring function for bullet symbol.
    pub fn without_bullet_color(&mut self) -> &mut Self {
        self.bullet_color = colored_string_from_str;
        self
    }
}

/// [`Bullet`] struct represents a bullet in a list item.
#[derive(Debug, Clone, PartialEq)]
pub enum Bullet<T: Display> {
    /// Numeric bullet from the `x` index ahead (1 => "1", "2", "3").
    Numeric(usize),
    /// Numeric bullet followed by a dot from the `x` index ahead (1 => "1.", "2.", "3.").
    NumericDot(usize),
    /// Numeric bullet followed by a parentheses from the `x` index ahead (1 => "1)", "2)", "3)").
    NumericParen(usize),
    /// Use the given symbol as bullet.
    Symbol(T),
}

impl<T: Display> Bullet<T> {
    /// Returns the symbol being used if self is a [`Bullet::Symbol`] variant, otherwise, returns
    /// [`None`].
    fn get_symbol(&self) -> Option<&T> {
        match self {
            Self::Symbol(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the numeric offset if self is a `Bullet::numeric` variant, otherwise, returns [`None`].
    fn get_num_offset(&self) -> Option<usize> {
        match self {
            Self::NumericDot(n) => Some(n),
            Self::NumericParen(n) => Some(n),
            Self::Numeric(n) => Some(n),
            _ => None,
        }
        .map(|n| n.to_owned())
    }
}

/// Struct returned when calling [`List::into_iter`]. This struct holds both the item and the
/// optional bullet. It can be easily printed with [`println!`] macro since it implements the
/// [`Display`] trait.
pub struct Item {
    /// The optional colored bullet.
    bullet: Option<ColoredString>,
    /// The current item from the list.
    item: ColoredString,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.bullet.as_ref() {
            Some(b) => write!(f, "{} {}", b, &self.item),
            None => write!(f, "{}", &self.item),
        }
    }
}

impl Item {
    /// returns the [`Item`]'s inner value (with no bullet) as [`str`] slice.
    fn get_item(&self) -> &str {
        &self.item
    }

    /// Returns the [`Item`]'s inner bullet (if exists) as an optional [`str`] slice.
    fn get_bullet(&self) -> Option<&str> {
        self.bullet.as_deref()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn items() {
        let mut list: List<&str, &str> = List::new();
        assert!(list.is_empty());
        assert!(!list.with_items(["item 1", "item 2"]).is_empty() && list.len() == 2);
        let mut iter = list.clone().into_iter();
        assert!(iter.next().is_some_and(|item| item.get_item() == "item 1"));
        assert!(iter.next().is_some_and(|item| item.get_item() == "item 2"));
        assert!(iter.next().is_none());
        assert!(list.without_items().is_empty());
    }

    #[test]
    fn bullet() {
        let mut list: List<&str, &str> = List::new();
        assert!(list.get_bullet().is_none());
        assert_eq!(
            list.with_bullet(Bullet::Symbol("*")).get_bullet(),
            Some(&Bullet::Symbol("*"))
        );
        assert!(list.get_bullet().is_some_and(
            |b| b.get_symbol().is_some_and(|s| *s == "*") && b.get_num_offset().is_none()
        ));
        assert_eq!(
            list.with_bullet(Bullet::Numeric(2))
                .get_bullet()
                .and_then(|b| b.get_num_offset()),
            Some(2)
        );
        let variants: [fn(usize) -> Bullet<&'static str>; 3] =
            [Bullet::Numeric, Bullet::NumericDot, Bullet::NumericParen];
        let expected = |n: usize, variant: fn(usize) -> Bullet<&'static str>| match variant(n) {
            Bullet::Numeric(_) => format!("{}", n),
            Bullet::NumericDot(_) => format!("{}.", n),
            Bullet::NumericParen(_) => format!("{})", n),
            x => unreachable!("this variant wasn't expected to happen: {x:?}"),
        };
        for (i, var) in variants.into_iter().enumerate() {
            list = List::new();
            list.with_items(["Good item", "Other Item", "How many items can we hold?"])
                .with_bullet(var(i));
            for (j, item) in list.into_iter().enumerate() {
                assert_eq!(item.get_bullet(), Some(expected(j + i, var)).as_deref());
            }
        }
    }
}
