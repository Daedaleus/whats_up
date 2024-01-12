use thiserror::Error;

#[derive(Error, Debug)]
pub enum GroupError {
    #[error("Could not parse group request")]
    RequestParseError,
    #[error("Could not find group")]
    NotFound,
    #[error("Could not add group to user")]
    AddError,
}
