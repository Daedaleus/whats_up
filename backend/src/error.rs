use thiserror::Error;

#[derive(Error, Debug)]
pub enum WhatsUpError {
    #[error("Could not parse group request")]
    GroupRequestParseError,
    #[error("Could not find group")]
    GroupNotFound,
    #[error("Could not add group to user")]
    GroupAddError,
}
