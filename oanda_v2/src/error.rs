use error_stack::{Report, ResultExt};
use parse_display::Display;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Display, Debug, Default)]
#[display(style = "snake_case")]
pub enum Error {
    /// Most errors come from this category
    #[default]
    General,
    #[display("Message: {}")]
    Message(String),
}

impl Error {
    pub fn new(msg: impl ToString) -> Error {
        Error::Message(msg.to_string())
    }
}

impl std::error::Error for Error {}
pub type Result<T> = error_stack::Result<T, Error>;

pub trait Take {
    fn take(self, msg: impl ToString) -> impl ResultExt;
}

impl<R: ResultExt> Take for R {
    fn take(self, msg: impl ToString) -> impl ResultExt {
        self.change_context_lazy(|| Error::new(msg.to_string()))
    }
}
