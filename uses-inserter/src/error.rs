#[derive(Debug, Default)]
pub enum Error {
    Message(String),
    #[default]
    General,
}

impl Error {
    pub fn new(msg: impl ToString) -> Error {
        Error::Message(msg.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(msg) => write!(f, "{msg}"),
            Error::General => write!(f, "General"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = error_stack::Result<T, Error>;
