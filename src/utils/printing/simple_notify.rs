use std::fmt::Display;

/// Unit struct that just prints to the terminal. Better having a handler than placing a
/// [`println`] macro all code ahead...
pub struct SimpleNotify;

impl SimpleNotify {
    /// Prints the message to the terminal. Specifies if it should be at stderr or not. (no new
    /// line).
    pub fn notify<T: Display>(stderr: bool, message: T) {
        if stderr {
            eprint!("{}", message);
        } else {
            print!("{}", message);
        }
    }

    /// Prints the message to the terminal. Specifies if it should be at stderr or not. (new
    /// line added).
    pub fn notify_line<T: Display>(stderr: bool, message: T) {
        if stderr {
            eprintln!("{}", message);
        } else {
            println!("{}", message);
        }
    }
}
