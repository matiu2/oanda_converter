use std::{
    fmt::{Debug, Display},
    marker::{Send, Sync},
};

use error_stack::{IntoReport as ESIntoReport, ResultExt};

/// An Empty error. It just represents that it's an error from
/// the medication_knowledge_client library
/// Further error message can be gotten from the error_stack library
#[derive(Debug, Default)]
pub enum Error {
    /// Most errors come from this category
    #[default]
    General,
    Message(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::General => write!(f, "General"),
            Error::Message(message) => write!(f, "Message: {message}"),
        }
    }
}

impl std::error::Error for Error {}
pub type Result<T> = error_stack::Result<T, Error>;

pub trait IntoReport<T> {
    /// Similar to [`error_stack::into_report`] but also changes the context of the error stack to our Error class
    fn into_report(self) -> Result<T>;
    /// Similar to [`error_stack::into_report`] but also changes the context of the error stack to our Error class
    /// and adds a attach_printable
    fn annotate(self, msg: impl Display + Debug + Send + Sync + 'static) -> Result<T>;
    /// Similar to [`error_stack::into_report`] but also changes the context of the error stack to our Error class
    /// and adds a attach_printable_lazy
    fn annotate_lazy<F, D>(self, f: F) -> Result<T>
    where
        F: Fn() -> D,
        D: Display + Debug + Send + Sync + 'static;
}

impl<T, E> IntoReport<T> for std::result::Result<T, E>
where
    std::result::Result<T, E>: ESIntoReport<Ok = T, Err = E>,
{
    #[track_caller]
    fn into_report(self) -> Result<T> {
        ESIntoReport::into_report(self).change_context(Error::General)
    }

    #[track_caller]
    fn annotate(self, msg: impl Display + Debug + Send + Sync + 'static) -> Result<T> {
        ESIntoReport::into_report(self)
            .change_context(Error::General)
            .attach_printable(msg)
    }

    #[track_caller]
    fn annotate_lazy<F, D>(self, f: F) -> Result<T>
    where
        F: Fn() -> D,
        D: Display + Debug + Send + Sync + 'static,
    {
        ESIntoReport::into_report(self)
            .change_context(Error::General)
            .attach_printable_lazy(f)
    }
}
