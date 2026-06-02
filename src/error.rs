use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Errore nella verifica della firma crittografica: {0}")]
    SignatureVerification(String),

    #[error("Errore nel parsing del JSON di Discord: {0}")]
    JsonParsing(#[from] serde_json::Error),

    #[error("Variabile d'ambiente mancante o invalida: {0}")]
    Environment(#[from] worker::Error),

    #[error("Il comando '{0}' non è stato registrato nel router")]
    CommandNotFound(String),
    
    #[error("Il payload ricevuto non è un'interazione valida")]
    InvalidInteraction,
}

pub type Result<T> = std::result::Result<T, Error>;