use colored::Colorize;

/// Turn text into it's weak form style.
pub trait WeakStyle: ToString {
    /// Turn text into it's weak style.
    fn weak_style(&self) -> String;
}

impl<T: ToString> WeakStyle for T {
    fn weak_style(&self) -> String {
        self.to_string().bright_black().to_string()
    }
}
