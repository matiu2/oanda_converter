use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};

use crate::definition_docs::Schema;

/// All the different endpoint types
#[derive(
    Display, Default, FromStr, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone, Copy,
)]
#[display(style = "snake_case")]
pub enum Endpoints {
    #[default]
    Account,
    Instrument,
    Order,
    Trade,
    Position,
    Transaction,
    Pricing,
}

/// The documentation for an API call from one of the endpoints
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RestCall {
    pub endpoint: Endpoints,
    pub http_method: HttpMethod,
    pub path: String,
    pub doc_string: String,
    pub parameters: Vec<RestCallParameter>,
    pub responses: Vec<Response>,
    /// Other reponses taht of type HTTPErrorResponse
    pub other_responses: Vec<u16>,
}

/// The documentation for a possible http error response.
/// Not an actual http error, just documention for it
/// See here: https://developer.oanda.com/rest-live-v20/troubleshooting-errors/#400
#[derive(Debug, Serialize, Deserialize)]
pub enum HTTPErrorResponse {
    ErrorMessage { error_message: String },
    RejectReason { reject_reason: String },
}

/// Encodes the documentation for a REST response given by an HTTP call to the Oanda API
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub code: u16,
    pub description: String,
    pub headers: Vec<ResponseHeader>,
    pub schema: Schema,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseHeader {
    pub name: String,
    pub description: String,
}

#[derive(Display, Default, FromStr, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[display(style = "UPPERCASE")]
pub enum HttpMethod {
    #[default]
    Get,
    Post,
    Put,
    Patch,
}

/// How a parameter should be sent to an http request
#[derive(Display, FromStr, PartialEq, Debug, Serialize, Deserialize)]
#[display(style = "snake_case")]
pub enum LocatedIn {
    Header,
    Path,
    Query,
}

impl LocatedIn {
    /// Returns true, only for header params
    pub fn is_header(&self) -> bool {
        matches!(self, LocatedIn::Header)
    }
    /// Returns true, only for path params
    pub fn is_path(&self) -> bool {
        matches!(self, LocatedIn::Path)
    }
    /// Returns true, only for query params
    pub fn is_query(&self) -> bool {
        matches!(self, LocatedIn::Query)
    }
}

/// An http request documentation body
#[derive(Debug, Serialize, Deserialize)]
pub struct RestCallParameter {
    pub name: String,
    pub located_in: LocatedIn,
    pub type_name: String,
    pub description: String,
}
