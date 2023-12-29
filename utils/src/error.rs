use error_stack::{Report, ResultExt};
use std::{
    fmt::Debug,
    marker::{Send, Sync},
};

/// An Empty error. It just represents that it's an error from
/// the medication_knowledge_client library
/// Further error message can be gotten from the error_stack library
#[derive(Debug)]
pub enum Error {
    Message(String),
}

impl Error {
    pub fn new(msg: impl ToString) -> Error {
        Error::Message(msg.to_string())
    }
}

impl std::error::Error for Error {}
pub type Result<T> = error_stack::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(message) => write!(f, "{message}"),
        }
    }
}

pub trait EasyError<T> {
    /// Similar to [`error_stack::into_report`] but also changes the context of the error stack to our Error class
    /// and adds a attach_printable
    fn annotate(self, msg: impl ToString) -> Result<T>;
    /// Similar to [`error_stack::into_report`] but also changes the context of the error stack to our Error class
    /// and adds a attach_printable_lazy
    fn annotate_lazy<F>(self, f: F) -> Result<T>
    where
        F: Fn() -> String;
}

pub trait Tracer {
    /// Just add a trace line
    fn trace(self) -> Self;
}

impl<T, E> EasyError<T> for std::result::Result<T, E>
where
    E: std::error::Error + Into<Report<E>> + std::marker::Send + Send + Sync + 'static,
{
    #[track_caller]
    fn annotate(self, msg: impl ToString) -> Result<T> {
        self.map_err(Report::from)
            .change_context(Error::Message(msg.to_string()))
    }

    #[track_caller]
    fn annotate_lazy<F>(self, f: F) -> Result<T>
    where
        F: Fn() -> String,
    {
        self.map_err(Report::from)
            .change_context(Error::Message(f()))
    }
}

impl<T> Tracer for Result<T> {
    #[track_caller]
    fn trace(self) -> Self {
        use std::panic::Location;
        let location = Location::caller();
        let file_name = location.file();
        let line_number = location.line();
        let column_number = location.column();
        self.attach_printable_lazy(|| format!("{file_name}: {line_number}:{column_number}"))
    }
}
