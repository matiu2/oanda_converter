use std::{error::Error as StdError, fmt::Display};

use scraper::error::SelectorErrorKind;

#[derive(Debug, Default)]
pub struct Error {
    pub message: Option<String>,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self {
            message: Some(message),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Web Scraper Error")
    }
}

impl StdError for Error {}

impl From<SelectorErrorKind<'_>> for Error {
    fn from(value: SelectorErrorKind) -> Self {
        Self::new(value.to_string())
    }
}
