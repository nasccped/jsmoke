/// Trim and box to [`str`].
pub trait TrimAndBox {
    /// Trim string and turn it into a boxed string.
    fn trim_and_box(&self) -> Box<str>;
}

impl<S: AsRef<str>> TrimAndBox for S {
    fn trim_and_box(&self) -> Box<str> {
        self.as_ref().trim().into()
    }
}
