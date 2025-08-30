use clap::ArgMatches;

/// This trait is morely used within test blocks for
/// [`clap::Command`] field data extraction.
///
/// Looks how it's straight forward:
///
/// ```
/// // this is better:
/// let inner_value = command.force_get_one(field_name);
/// // than this:
/// let inner_value = command
///     .get_one::<String>(field_name)
///     .map(|val| val.to_string())
///     .unwrap();
/// ```
///
/// ## Warning
///
/// _"Force"_ functions will force the data extraction. If something
/// goes wrong, the entire program will panic!
#[allow(dead_code)]
pub trait ArgMatchesUtil {
    fn force_get_one(&self, field: &str) -> String;
}

impl ArgMatchesUtil for ArgMatches {
    fn force_get_one(&self, field: &str) -> String {
        match self.get_one::<String>(field) {
            Some(value) => value.to_string(),
            _ => panic!(r#""{field}" field is missing"#),
        }
    }
}
