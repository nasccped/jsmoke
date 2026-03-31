/// Public trait for message notify based on low priority objects.
pub trait SimpleNotify {
    /// Notify info based on `self` reference.
    fn notify(&self);
}
