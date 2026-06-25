use std::fmt::Display;

/// Style as list of items.
///
/// The list can be either `ordered` or not and can hold any implementor of `'static` (current
/// block lifetime) and [`Display`] trait.
pub struct ListStyle {
    /// If it's ordered.
    ordered: bool,

    /// Padding from terminal's left side.
    left_padding: usize,

    /// Items itself.
    list: Vec<Box<dyn Display>>,
}

impl ListStyle {
    /// Creates a new **unordered** list of items.
    pub fn new_unordered() -> Self {
        Self::new(false)
    }

    /// Creates a new **ordered** list of items.
    pub fn new_ordered() -> Self {
        Self::new(true)
    }

    /// Private function that creates a new item based on `ordered` param.
    #[inline]
    fn new(ordered: bool) -> Self {
        Self {
            ordered,
            left_padding: 0,
            list: Vec::new(),
        }
    }

    /// If the list is ordered.
    #[inline]
    pub fn is_ordered(&self) -> bool {
        self.ordered
    }

    /// Push a single item to the list.
    ///
    /// Item can be any type since it implements `'static` + [`Display`] trait.
    pub fn push_item(&mut self, item: impl Display + 'static) {
        self.list.push(Box::new(item));
    }

    /// Push a sequence of items to the list.
    ///
    /// `items` must be an array of any type that implements `'static` + [`Display`] trait.
    ///
    /// An array can holds only single type elements, so, all the data within it must be the same.
    /// Alternatively you can use [`ListStyle::push_item`] to alternate between types.
    pub fn push_items<const N: usize>(&mut self, items: [impl Display + 'static; N]) {
        for i in items {
            self.list.push(Box::new(i));
        }
    }

    /// Set a new left padding value.
    #[inline]
    pub fn set_left_padding(&mut self, padding: usize) {
        self.left_padding = padding;
    }

    /// Returns the left padding value.
    #[inline]
    pub fn padding(&self) -> usize {
        self.left_padding
    }

    /// Consumes the self item by returning the inner list.
    #[inline]
    pub fn consume(self) -> Vec<Box<dyn Display>> {
        self.list
    }
}
