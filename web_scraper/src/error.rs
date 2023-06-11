use error_stack::{IntoReport as ESIntoReport, ResultExt};
use parse_display::Display;

/// An Empty error. It just represents that it's an error from
/// the web-scraper library
/// Further error message can be gotten from the error_stack library
#[derive(Debug, Default, Display)]
pub enum Error {
    /// Most errors come from this category
    #[default]
    General,
    #[display("{0}")]
    Message(String),
}

impl std::error::Error for Error {}
pub type Result<T> = error_stack::Result<T, Error>;
pub type HttpResult<T> = std::result::Result<T, Error>;

pub trait IntoReport<T> {
    fn into_report(self) -> Result<T>;
}

impl<T, E> IntoReport<T> for std::result::Result<T, E>
where
    std::result::Result<T, E>: ESIntoReport<Ok = T, Err = E>,
{
    #[track_caller]
    fn into_report(self) -> Result<T> {
        ESIntoReport::into_report(self).change_context(Error::General)
    }
}

impl From<scraper::error::SelectorErrorKind<'_>> for Error {
    fn from(value: scraper::error::SelectorErrorKind) -> Self {
        // Scraper errors include refreneces to html and lifetime management.
        // error_stack::Report doesn't
        // We just render the error out
        Error::Message(format!("{value:#?}"))
    }
}
