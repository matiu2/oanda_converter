use std::fmt::Display;

#[derive(Default, Debug)]
pub enum Error {
    #[default]
    Standard,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Standard => write!(f, "Standard"),
        }
    }
}

impl std::error::Error for Error {}
