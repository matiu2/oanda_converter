pub mod defintion_docs;
pub mod endpoint_docs;
use defintion_docs::Definition;
use endpoint_docs::RestCall;
use serde::{Deserialize, Serialize};
use url::Url;

//// The content of one page of the oanda docs
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub urls: Vec<Url>,
    pub documentation: Documentation,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Documentation {
    Endpoint(Vec<RestCall>),
    Definitions(Vec<Definition>),
}
