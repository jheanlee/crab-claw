use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    InvalidParameter(String),
    IoError(tokio::io::Error),
    JsonError(serde_json::Error),
    NonUTF8PathName,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParameter(error_message) => write!(f, "InvalidParameter {error_message}"),
            Self::IoError(error) => write!(f, "IoError: {error}"),
            Self::JsonError(error) => write!(f, "JsonError: {error}"),
            Self::NonUTF8PathName => write!(f, "NonUTF8PathName"),
        }
    }
}

impl std::error::Error for Error {}

impl From<tokio::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(error)
    }
}
