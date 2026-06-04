use hex::FromHexError;
use thiserror::Error;
use worker::Response;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Header '{0}' not found.")]
    MissingHeader(String),

    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    #[error("Invalid interaction: {0}")]
    InvalidInteraction(String),

    #[error("Invalid public key or signature format: {0:?}")]
    CryptoError(#[from] ed25519_dalek::SignatureError),

    #[error("Failed to parse from hex: {0:?}")]
    ParseHexFailed(#[from] FromHexError),

    #[error("JSON error: {0:?}")]
    JsonFailed(#[from] serde_json::Error),

    #[error("Worker error: {0}")]
    WorkerError(#[from] worker::Error),

    #[error("Environment variable '{0}' not found.")]
    EnvironmentVariableNotFound(String),

    #[error("Command '{0}' is not registered")]
    CommandNotFound(String),

    #[error("Modal '{0}' is not registered")]
    ModalNotFound(String),

    #[error("Component '{0}' is not registered")]
    ComponentNotFound(String),

    #[error("Execute not implemented for command: '{0}'")]
    ExecuteNotImplemented(String),

    #[error("Autocomplete not implemented for comand: '{0}'")]
    AutocompleteNotImplemented(String),

    #[error("Error communicating with {0}")]
    UpstreamError(String),

    #[error("Error: {0}")]
    Generic(String),
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