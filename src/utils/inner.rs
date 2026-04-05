/// Trait to borrow the inner data (as reference) from complex types. A lot useful for tuple
/// structs to avoid the `Struct(pub item)` thing.
pub trait Inner<T: ?Sized> {
    /// Extracts the item inner data as reference.
    fn inner(&self) -> &T;
}

/// Works similar to [`Inner`] trait but returns the owned inner value + consume the `self` item.
pub trait InnerAndConsume<T> {
    /// Extracts the item inner as owned value + consume the `self` item.
    fn inner_and_consume(self) -> T;
}
