use parse_display::Display;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use thiserror::Error as ThisError;
#[derive(ThisError, Display, Debug)]
#[display(style = "snake_case")]
pub enum Error {
    #[display("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[display("Unexpected http error: {code} {body}")]
    UnexpectedHttp { code: u16, body: String },
}
impl std::error::Error for Error {}
pub type Result<T> = error_stack::Result<T, Error>;
