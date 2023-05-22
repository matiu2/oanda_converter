use crate::{annotate, bail, Error, Result};
use codegen::Impl;
use codegen::Scope;
use convert_case::{Case, Casing};
use model::endpoint_docs::{Endpoint, RestCall};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

mod rest_call_ext;
use rest_call_ext::RestCallExt;

/// Takes an array of RestCalls and generates the rust code.
///
/// They must all have the same endpoint, and they will all be in generated in the same file.
/// The filename is the module name in snake_case with a `.rs` extensnion.
/// For example Endpoint::Account -> "{dir}/account.rs"
///
/// Returns the name of the module so you can put in in: `mod {name}`. For the above example it'll return `account`
///
///
/// # Errors
///
/// This function will return an error if it can't create the files or parse or convert any of the input
pub fn create_endpoint(dir: &Path, rest_calls: &[RestCall]) -> Result<Option<String>> {
    // All the rest_calls should have the same endpoint.
    // We're making one rust module per Oanda API endpoint.
    let Some(first_rest_call) = rest_calls.first() else { bail!("Can't create endpoint when there are no RestCalls: {dir:#?}")};
    let endpoints = rest_calls.iter().map(|call| call.endpoint);
    if endpoints
        .clone()
        .any(|endpoint| endpoint != first_rest_call.endpoint)
    {
        let endpoints: HashSet<Endpoint> = endpoints.collect();
        bail!("We expected all the rest_calls in an endpoint to have the same endpoint. This is necessary because we create one rust module per endpoint. We found: {endpoints:#?}");
    };

    let endpoint_name = first_rest_call.endpoint_name();

    // Create the scope to write the code
    let mut scope = Scope::new();

    // Import everything we need
    scope.import("error_stack", "Result");
    scope.import("error_stack", "ResultExt");
    scope.import("crate::client", "Client");
    scope.import("crate::error", "Error");

    // Generate the endpoint as a struct
    let r#struct = scope.new_struct(endpoint_name.as_str());
    r#struct
        .vis("pub")
        .generic("'a")
        .field("client", "&'a Client")
        .vis("pub(crate)");

    // Create the parameter type imports for all the rest_calls
    let param_imports: HashSet<&str> = rest_calls
        .iter()
        .flat_map(|call| call.parameters.iter())
        .map(|param| param.type_name.as_str())
        .collect();
    param_imports.into_iter().for_each(|import| {
        scope.import("crate::definitons", import);
    });

    // Create the implementation for the struct
    let mut r#impl = scope.new_impl(&format!("{endpoint_name}")).generic("'a");

    for call in rest_calls {
        // Import all the types for the rest call
        create_rest_call(&mut r#impl, call)?
    }

    // TODO: Create the Request and Response Types for each call if any

    #[cfg(test)]
    crate::print_code(scope.to_string().as_str());

    // Save the module
    let mod_name = first_rest_call.endpoint_name().to_case(Case::Snake);
    let mut file_name = PathBuf::new();
    file_name.push(dir);
    file_name.push(format!("{}.rs", mod_name));
    annotate!(
        std::fs::write(file_name.as_path(), scope.to_string()),
        "Writing module {file_name:#?}"
    )?;

    Ok(Some(mod_name))
}

/// Generates the code for a single REST api call.
pub fn create_rest_call(r#impl: &mut Impl, call: &RestCall) -> Result<()> {
    // TODO: Figure out the return type from call.responses
    let fun = r#impl
        .new_fn(call.method_name()?.as_str())
        .doc(&call.doc_string)
        .vis("pub");

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{annotate, Result};
    use indoc::indoc;
    use model::endpoint_docs::RestCall;
    use tempfile::tempdir;

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

    fn test_rest_call() -> Result<RestCall> {
        annotate!(serde_yaml::from_str(REST_CALL), "parse yaml")
    }

    #[test]
    fn test_create_endpoint() -> Result<()> {
        let test_rest_call = test_rest_call()?;
        let dir = annotate!(tempdir(), "Creating temp dir")?;
        let module_name = super::create_endpoint(dir.path(), &[test_rest_call])?;
        assert_eq!(module_name, Some("account".to_string()));
        Ok(())
    }
}
