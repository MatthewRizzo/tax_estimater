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
    #[error("Errors due to the server internally failing")]
    ServerError(String),
    #[error("Errors due to the an incorrect income being used for a bracket")]
    BracketError(BracketErrors),
    #[error("Errors due to the a file not existing.")]
    FileError(String),
    #[error("Errors due to serde deserializing a file.")]
    SerdeDeserializeError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum BracketErrors {
    #[error("Errors due to the income not meeting minimum requirement of a bracket.")]
    SmallIncomeError(String),
    #[error("Errors due to the income exceeding the maximum allowed by a bracket.")]
    LargeIncomeError(String),
    #[error("Errors due to tax rate not being within [0, 1].")]
    TaxRateError(String),
    #[error("Errors due to bracket min and max")]
    RangeError(String)
}
