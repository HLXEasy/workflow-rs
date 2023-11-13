use reqwest::Error as ReqwestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Custom: {0}")]
    Custom(String),

    #[error("Reqwest: {0}")]
    Reqwest(#[from] ReqwestError),

    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Custom(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Custom(s.to_string())
    }
}

impl Error {
    pub fn custom<T: ToString>(s: T) -> Self {
        Error::Custom(s.to_string())
    }
}
