pub mod definition_docs;
pub mod endpoint_docs;
pub use definition_docs::Definition;
use endpoint_docs::RestCall;
use serde::{Deserialize, Serialize};
use url::Url;

/// Represents all the documentation in the oanda Rest API reference
#[derive(Debug, Serialize, Deserialize)]
pub struct Everything {
    /// Each endpoint reference documentation or definition collection from a single page in the website.
    /// eg. https://developer.oanda.com/rest-live-v20/account-ep/
    /// eg. https://developer.oanda.com/rest-live-v20/instrument-df/
    pub content: Vec<Content>,
    /// This is scraped from https://developer.oanda.com/rest-live-v20/troubleshooting-errors
    pub errors: Vec<ErrorDefinition>,
}

/// Represents a single error definition from https://developer.oanda.com/rest-live-v20/troubleshooting-errors/
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDefinition {
    pub code: u16,
    pub documentation: String,
}

/// The content of one page of the oanda docs
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub urls: Vec<Url>,
    pub documentation: Documentation,
}

impl Content {
    /// If this is an endpoint, returns all of its RestCalls, otherwise returns nothing
    #[deprecated(note = "Probably can be deleted because we have `as_endpoint` now")]
    pub fn calls(&self) -> Option<&Vec<RestCall>> {
        match &self.documentation {
            Documentation::Endpoint(Endpoint { calls, .. }) => Some(calls),
            Documentation::Definitions { .. } => None,
        }
    }

    /// Returns all the definitions if there are any, otherwise None
    pub fn definitions(&self) -> Option<&Vec<Definition>> {
        match &self.documentation {
            Documentation::Endpoint { .. } => None,
            Documentation::Definitions { definitions, .. } => Some(definitions),
        }
    }

    /// If this page content represents an endpoint, returns that info
    pub fn as_endpoint(&self) -> Option<&Endpoint> {
        match &self.documentation {
            Documentation::Endpoint(endpoint) => Some(endpoint),
            Documentation::Definitions { .. } => None,
        }
    }

    pub fn name(&self) -> &str {
        match &self.documentation {
            Documentation::Endpoint(Endpoint { name, .. }) => name,
            Documentation::Definitions { name, .. } => name,
        }
    }
}

/// Represents a single api endpoint definition. eg. https://developer.oanda.com/rest-live-v20/instrument-df/
#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub name: String,
    pub calls: Vec<RestCall>,
}

/// Each page of the oanda website is either
/// A bunch of definitions. eg. https://developer.oanda.com/rest-live-v20/instrument-df/
/// Or a single endpoint with multiple Rest calls. eg. https://developer.oanda.com/rest-live-v20/account-ep/
#[derive(Debug, Serialize, Deserialize)]
pub enum Documentation {
    Endpoint(Endpoint),
    Definitions {
        name: String,
        definitions: Vec<Definition>,
    },
}
