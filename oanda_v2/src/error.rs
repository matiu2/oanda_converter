use parse_display::Display;
use thiserror::Error as ThisError;
use reqwest::StatusCode;
use serde::Deserialize;
#[derive(Display, Debug, ThisError)]
#[display(style = "snake_case")]
pub enum Error {
    #[display("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[display("Unexpected http error: {code} {body}")]
    UnexpectedHttp { code: u16, body: String },
}
pub type Result<T> = error_stack::Result<T, Error>;
