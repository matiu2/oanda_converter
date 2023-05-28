use crate::shared::write_struct_fields;
use crate::{bail, Error, Result};
use codegen::Impl;
use codegen::Module;
use codegen::Scope;
use model::defintion_docs::Schema;
use model::endpoint_docs::{Endpoint, RestCall};
use std::{collections::HashSet, path::Path};

mod rest_call_ext;
use rest_call_ext::RestCallExt;

/// Takes an array of RestCalls and generates the rust code.
///
/// They must all have the same endpoint, and they will all be in generated in the same file.
/// The filename is the module name in snake_case with a `.rs` extensnion.
/// For example Endpoint::Account -> "{dir}/account.rs"
///
/// We're making one rust module per Oanda API endpoint.
///
/// Returns the generated code
///
/// # Errors
///
/// This function will return an error if it can't create the files or parse or convert any of the input
pub fn create_endpoint(dir: &Path, rest_calls: &[RestCall]) -> Result<Scope> {
    // All the rest_calls should have the same endpoint.
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

    // Generate modules that encapsulate the request and response types
    // That are not already in the definitions section
    // TODO: Maybe move this into its own sub_file
    let mut request_types = scope.new_module("request_types");
    // create_request_types(&mut request_types, rest_calls)?;
    let mut response_types = scope.new_module("response_types");
    create_response_types(&mut response_types, rest_calls)?;

    // Import everything we need
    scope.import("crate", "Result");
    scope.import("crate", "annotate");
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
    let r#impl = scope.new_impl(&endpoint_name.to_string()).generic("'a");

    for call in rest_calls {
        // Import all the types for the rest call
        create_rest_call(r#impl, call)?
    }

    // TODO: Create the Request and Response Types for each call if any

    #[cfg(test)]
    crate::print_code(scope.to_string().as_str());

    Ok(scope)
}

/// Generates the code for a single REST api call.
pub fn create_rest_call(r#impl: &mut Impl, call: &RestCall) -> Result<()> {
    let fun = r#impl
        .new_fn(call.method_name()?.as_str())
        .doc(&call.doc_string)
        .vis("pub")
        // TODO: Request types
        .ret(format!("Result<responses::{}>", call.return_name()?));
    fun.line(format!(r#"let url = self.client.url("{}");"#, &call.path));
    match call.http_method {
        model::endpoint_docs::HttpMethod::Get => {
            fun.line("let request = self.client.start_get(&url);")
        }
        model::endpoint_docs::HttpMethod::Post => todo!(),
        model::endpoint_docs::HttpMethod::Put => todo!(),
        model::endpoint_docs::HttpMethod::Patch => todo!(),
    };
    fun.line("self.client.get(request).await.attach_printable_lazy(call.method_name)");
    Ok(())
}

/// Each of the rest calls has unique request parameter types that haven't been
/// defined in the definition section. We'll define them here under the actual API endpoint itself
fn create_request_types(scope: &mut Module, rest_calls: &[RestCall]) -> Result<()> {
    todo!()
}

/// For all of the rest calls, generate the Rust types that they'll return
fn create_response_types(mut module: &mut Module, rest_calls: &[RestCall]) -> Result<()> {
    for call in rest_calls {
        // Create the struct for each response for this call
        for response in &call.responses {
            let mut r#struct = module
                .new_struct(&format!("{}{}", call.return_name()?, response.code))
                .doc(&response.description);
            match &response.schema {
                Schema::Struct(s) => write_struct_fields(&mut r#struct, &s),
                Schema::Stream(s) => todo!(),
            }
        }

        // Create the enum that wraps the structs
        let mut r#enum = module.new_enum(&call.return_name()?).doc(&format!(
            "Response type for {}::{}",
            call.endpoint_name(),
            call.method_name()?
        ));

        // Generate each enum variant
        for response in &call.responses {
            r#enum
                .new_variant(format!("Code{}", response.code))
                .tuple(&format!("{}{}", call.return_name()?, response.code));
        }
    }
    Ok(())
}

/// Given Stream from a model, generates the definition code
fn write_stream(code: &mut codegen::Struct, input: &model::defintion_docs::Stream) -> Result<()> {
    todo!()
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
        let code = super::create_endpoint(dir.path(), &[test_rest_call])?;
        let code = code.to_string();
        println!("{code}");
        assert!(code.contains("pub fn list_all()"));
        Ok(())
    }
}
