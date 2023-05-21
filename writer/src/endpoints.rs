use crate::{annotate, bail, Error, Result};
use codegen::Scope;
use convert_case::{Case, Casing};
use error_stack::IntoReport;
use error_stack::ResultExt;
use model::endpoint_docs::{Endpoint, RestCall};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

/// Takes an array of RestCalls and generates the rust code.
///
/// They must all have the same endpoint, and they will all be in generated in the same file.
/// The filename is the module name in snake_case with a `.rs` extensnion.
/// For example Endpoint::Account -> "{dir}/account.rs"
///
/// # Errors
///
/// This function will return an error if it can't create the files or parse or convert any of the input
pub fn create_endpoint(dir: &Path, rest_calls: &[RestCall]) -> Result<Option<PathBuf>> {
    // All the rest_calls should have the same endpoint.
    // We're making one rust module per Oanda API endpoint.
    let endpoints = rest_calls.iter().map(|call| call.endpoint);
    let Some(first_endpoint) = endpoints.clone().next() else { return Ok(None) };
    if endpoints.clone().any(|endpoint| endpoint != first_endpoint) {
        let endpoints: HashSet<Endpoint> = endpoints.collect();
        bail!("We expected all the rest_calls in an endpoint to have the same endpoint. This is necessary because we create one rust module per endpoint. We found: {endpoints:#?}");
    };

    // Create the scope to write the code
    let mut scope = Scope::new();

    for call in rest_calls {
        create_rest_call(&mut scope, call)?
    }

    // Save the module
    let mod_name = format!("{}.rs", first_endpoint.to_string().to_case(Case::Snake));
    let mut file_name = PathBuf::new();
    file_name.push(dir);
    file_name.push(mod_name);
    annotate!(
        std::fs::write(file_name.as_path(), scope.to_string()),
        "Writing module {file_name:#?}"
    )?;

    Ok(Some(file_name))
}

pub fn create_rest_call(scope: &mut Scope, call: &RestCall) -> Result<()> {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{annotate, Error, Result};
    use error_stack::IntoReport;
    use indoc::indoc;
    use model::endpoint_docs::Endpoint;

    const REST_CALL: &str = indoc!("
    endpoint: Account
    http_method: Get
    path: /v3/accounts
    doc_string: Get a list of all Accounts authorized for the provided token.
    parameters:
    - name: Authorization
      located_in: Header
      type_name: string
      description: The authorization bearer token previously obtained by the client [required]
    responses:
    - code: 200
      description: The list of authorized Accounts has been provided.
      headers:
      - name: RequestID
        description: The unique identifier generated for the request
      schema: !Struct
        fields:
        - name: accounts
          type_name: AccountProperties
          doc_string: The list of Accounts the client is authorized to access and their associated properties.
          is_array: true
          default: null
          required: false
    other_responses:
    - 401
    - 405
");

    fn test_endpoint() -> Result<Endpoint> {
        annotate!(serde_yaml::from_str(REST_CALL), "parse yaml")
    }

    #[test]
    fn test_create_endpoint() -> Result<()> {
        let test_endpoint = test_endpoint()?;
        todo!()
    }
}
