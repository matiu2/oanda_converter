use crate::Result;
use convert_case::{Case, Casing};
use model::endpoint_docs::RestCall;
mod method_namer;
use method_namer::method_name;

pub trait RestCallExt {
    /// Returns the endpoint name in PascalCase
    fn endpoint_name(&self) -> String;
    /// Returns the endpoint name in snake_case
    fn module_name(&self) -> String;
    /// Generates a method name for this REST call
    fn method_name(&self) -> Result<String>;
    /// The type that the api call returns. Later it will be wrapped in a result
    fn return_name(&self) -> Result<String>;
}

impl RestCallExt for RestCall {
    fn endpoint_name(&self) -> String {
        self.endpoint.to_string().to_case(Case::Pascal)
    }
    fn module_name(&self) -> String {
        self.endpoint.to_string().to_case(Case::Snake)
    }
    fn method_name(&self) -> Result<String> {
        method_name(self)
    }
    fn return_name(&self) -> Result<String> {
        Ok(format!(
            "{}Return",
            self.method_name()?.to_case(Case::Pascal)
        ))
    }
}
