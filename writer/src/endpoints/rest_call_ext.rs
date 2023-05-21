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
}
