use thiserror::Error;
use worker::Response;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error verifing signature: {0}")]
    SignatureVerification(String),

    #[error("Error parsing JSON: {0}")]
    JsonParsing(#[from] serde_json::Error),

    #[error("Could not find environment variable: {0}")]
    Environment(#[from] worker::Error),

    #[error("Command '{0}' is not registered")]
    CommandNotFound(String),
    
    #[error("The received payload is not a valid Interaction")]
    InvalidInteraction(),
}

impl Error {
    pub fn as_response(self) -> worker::Result<Response> {
        Response::empty()
    }
}

impl From<Error> for worker::Error {
    fn from(value: Error) -> Self {
        Self::from(format!("[rsflarecord] Error: {}", value))
    }
}

pub type Result<T> = std::result::Result<T, Error>;