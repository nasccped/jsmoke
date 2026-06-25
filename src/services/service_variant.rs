use super::new_service::NewService;

/// Which service must be ran.
pub enum AppServiceVariant<'a> {
    /// `new` service (associated with [`crate::cli::subcommands::New`]).
    New(NewService<'a>),
}

impl<'a> From<NewService<'a>> for AppServiceVariant<'a> {
    fn from(value: NewService<'a>) -> Self {
        Self::New(value)
    }
}
