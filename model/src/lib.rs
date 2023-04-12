use parse_display::{Display, FromStr};

/// All the different endpoint types
#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "snake_case")]
pub enum Endpoint {
    Account,
    Instrument,
    Order,
    Trade,
    Position,
    Transaction,
    Pricing,
}

/// The documentation for an API call from one of the endpoints
#[derive(Debug)]
pub struct RestCall {
    pub endpoint: Endpoint,
    pub http_method: HttpMethod,
    pub path: String,
    pub doc_string: String,
    pub parameters: Vec<RestCallParameter>,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
}

/// How a parameter should be sent to an http request
#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "snake_case")]
pub enum LocatedIn {
    Header,
    Path,
    Query,
}

/// An http request documentation body
#[derive(Debug)]
pub struct RestCallParameter {
    pub name: String,
    pub located_in: LocatedIn,
    pub type_name: String,
    pub description: String,
}
