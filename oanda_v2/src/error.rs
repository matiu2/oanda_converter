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
    #[display("API Error: {status} {error}")]
    Api { status: StatusCode, error: ApiError },
}
#[derive(Display, Deserialize, Debug)]
#[display(
    r#"error_codes: {error_codes:#?}
        error_uri: {error_uri}
        error_description: {error_description}
        trace_id: {trace_id}
        timestamp: {timestamp}
        error: {error}
        correlation_id: {correlation_id}"#
)]
pub struct ApiError {
    pub error_codes: Vec<usize>,
    pub error_uri: String,
    pub error_description: String,
    pub trace_id: String,
    pub timestamp: String,
    pub error: String,
    pub correlation_id: String,
}
impl std::error::Error for Error {}
pub type Result<T> = error_stack::Result<T, Error>;
