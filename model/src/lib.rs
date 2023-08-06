pub mod definition_docs;
pub mod endpoint_docs;
pub use definition_docs::Definition;
use endpoint_docs::RestCall;
use serde::{Deserialize, Serialize};
use url::Url;

//// The content of one page of the oanda docs
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub urls: Vec<Url>,
    pub documentation: Documentation,
}

impl Content {
    /// If this is an endpoint, returns all of its RestCalls, otherwise returns nothing
    pub fn calls(&self) -> Option<&Vec<RestCall>> {
        match &self.documentation {
            Documentation::Endpoint { calls, .. } => Some(calls),
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

    pub fn name(&self) -> &str {
        match &self.documentation {
            Documentation::Endpoint { name, .. } => name,
            Documentation::Definitions { name, .. } => name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Documentation {
    Endpoint {
        name: String,
        calls: Vec<RestCall>,
    },
    Definitions {
        name: String,
        definitions: Vec<Definition>,
    },
}

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => { return error_stack::IntoReport::into_report(Err(Error::Message(format!(($arg)*)))) };
}
