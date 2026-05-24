use crate::shared_models::reserveds::JavaReserveds;

/// Possible errors for [`super::ArtifactWrapper::try_from`] action.
#[derive(thiserror::Error, Debug)]
pub enum ArtifactWrapperParseError {
    #[error("empty strings aren't valid as artifact name")]
    Empty,
    #[error("string length isn't allowed for an artifact name")]
    Length(usize),
    #[error("the provided string isn't a valid artifact name ({})", .0)]
    InvalidPattern(String),
    #[error("the provided artifact name is a java's reserved word ({})", .0.to_string())]
    Reserved(JavaReserveds),
}
