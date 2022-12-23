use thiserror::Error;

pub type EstimaterResult<T> = std::result::Result<T, EstimaterErrors>;

#[derive(Error, Debug)]
pub enum EstimaterErrors {
    // #[error("Errors relating to a client interfacing with server")]
    // ClientErrors(String),
    // #[error("Errors internal to the client itself")]
    // InternalError(String),
    #[error("Errors Parsing Config File")]
    ParsingError(String),
    #[error("Errors due to user input")]
    UserError(String),
}
