use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    InvalidChoiceCount,
    LLMError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError(error) => write!(f, "RequestError: {error}"),
            Self::InvalidChoiceCount => write!(f, "LLM responded with an invalid choice count"),
            Self::LLMError(response) => {
                write!(f, "LLM has returned an unexpected response: {response}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}
