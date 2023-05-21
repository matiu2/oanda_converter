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

#[cfg(test)]
fn print_code(code: &str) {
    use syntect::easy::HighlightLines;
    use syntect::highlighting::{Style, ThemeSet};
    use syntect::parsing::SyntaxSet;
    use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(code) {
        // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

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
    let endpoints = rest_calls.iter().map(|call| call.endpoint);
    let Some(first_endpoint) = endpoints.clone().next() else { return Ok(None) };
    if endpoints.clone().any(|endpoint| endpoint != first_endpoint) {
        let endpoints: HashSet<Endpoint> = endpoints.collect();
        bail!("We expected all the rest_calls in an endpoint to have the same endpoint. This is necessary because we create one rust module per endpoint. We found: {endpoints:#?}");
    };

    let endpoint_name = first_endpoint.to_string().to_case(Case::Pascal);

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

    // Create the implementation for the struct
    scope.new_impl(&format!("{endpoint_name}")).generic("'a");

    #[cfg(test)]
    print_code(scope.to_string().as_str());

    for call in rest_calls {
        create_rest_call(&mut scope, call)?
    }

    // Save the module
    let mod_name = first_endpoint.to_string().to_case(Case::Snake);
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
pub fn create_rest_call(scope: &mut Scope, call: &RestCall) -> Result<()> {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{annotate, Error, Result};
    use error_stack::{IntoReport, ResultExt};
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
