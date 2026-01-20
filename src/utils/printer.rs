use std::error::Error;

/// Where to print the content.
#[derive(Default)]
enum FileTarget {
    /// Standard output as target file.
    #[default]
    Stdout,
    /// Standard error as target file.
    Stderr,
    // TODO: add a better `other` variant.
    #[allow(dead_code)]
    Other,
}

/// Tag that indicates an error title.
const ERR_TAG: &str = "\x1b[91merror\x1b[97m:\x1b[0m";

/// A general printing machine (stdout, stderr and other variants).
///
/// This struct implements the [`Default`] trait which automatically points to stdout file.
/// All the printed content will be sent to the file being pointed.
#[derive(Default)]
pub struct Printer {
    target: FileTarget,
}

impl Printer {
    /// Sets the target file to stderr.
    pub fn use_stderr(&mut self) -> &mut Self {
        self.target = FileTarget::Stderr;
        self
    }

    /// Prints the provided content into the target file with a new line.
    pub fn println<C: std::fmt::Display>(&self, content: C) {
        match self.target {
            FileTarget::Stdout => println!("{}", content),
            FileTarget::Stderr => eprintln!("{}", content),
            FileTarget::Other => unreachable!("TODO: implement other variant printing"),
        }
    }

    /// Prints the provided content into the target file without a new line.
    pub fn print<C: std::fmt::Display>(&self, content: C) {
        match self.target {
            FileTarget::Stdout => print!("{}", content),
            FileTarget::Stderr => eprint!("{}", content),
            FileTarget::Other => unreachable!("TODO: implement other variant printing"),
        }
    }

    /// Go to the next line without printing anything. Works the same as [`println!`] with no
    /// arguments (but still necessary since simple functions doesn't allow optional arguments).
    pub fn empty_line(&self) {
        match self.target {
            FileTarget::Stdout => println!(),
            FileTarget::Stderr => eprintln!(),
            FileTarget::Other => unreachable!("TODO: implement other variant printing"),
        }
    }

    /// Prints the error description with an error kind tag at beginning.
    pub fn print_err_tag<T: Error>(&mut self, err: T) {
        self.use_stderr();
        self.print(ERR_TAG);
        self.print(' ');
        self.println(err);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stdout_to_stderr() {
        let mut p = Printer::default();
        assert!(matches!(p.target, FileTarget::Stdout));
        assert!(matches!(p.use_stderr().target, FileTarget::Stderr));
    }
}
