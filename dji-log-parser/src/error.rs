use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid Api Key")]
    ApiKeyError,

    #[error("Api Key or keychain is required")]
    InvalidDecryptMethod,

    #[error("Missing Auxilliary data: {0}")]
    MissingAuxilliaryData(String),

    #[error("Parse error: {0}")]
    Parse(#[from] binrw::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    Network(#[from] ureq::Error),

    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
}
