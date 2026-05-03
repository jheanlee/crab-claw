use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    SendError,
    StdIoError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SendError => write!(f, "SendError"),
            Self::StdIoError(error) => write!(f, "IoError: {error}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::StdIoError(value)
    }
}
