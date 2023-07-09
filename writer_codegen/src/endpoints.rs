use crate::shared::write_struct_fields;
use crate::{bail, Error, Result};
use codegen::{Impl, Module, Scope};
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
        let code = super::create_endpoint(dir.path(), &[test_rest_call])?;
        let code = code.to_string();
        println!("{code}");
        assert!(code.contains("pub fn list_all()"));
        Ok(())
    }

    #[test]
    fn test_request_builder() -> Result<()> {
        let input = indoc!("
      endpoint: Pricing
      http_method: Get
      path: /v3/accounts/{accountID}/instruments/{instrument}/candles
      doc_string: Fetch candlestick data for an instrument.
      parameters:
      - name: Authorization
        located_in: Header
        type_name: string
        description: The authorization bearer token previously obtained by the client [required]
      - name: Accept-Datetime-Format
        located_in: Header
        type_name: AcceptDatetimeFormat
        description: Format of DateTime fields in the request and response.
      - name: accountID
        located_in: Path
        type_name: AccountID
        description: Account Identifier [required]
      - name: instrument
        located_in: Path
        type_name: InstrumentName
        description: Name of the Instrument [required]
      - name: price
        located_in: Query
        type_name: PricingComponent
        description: The Price component(s) to get candlestick data for. [default=M]
      - name: granularity
        located_in: Query
        type_name: CandlestickGranularity
        description: The granularity of the candlesticks to fetch [default=S5]
      - name: count
        located_in: Query
        type_name: integer
        description: The number of candlesticks to return in the response. Count should not be specified if both the start and end parameters are provided, as the time range combined with the granularity will determine the number of candlesticks to return. [default=500, maximum=5000]
      - name: from
        located_in: Query
        type_name: DateTime
        description: The start of the time range to fetch candlesticks for.
      - name: to
        located_in: Query
        type_name: DateTime
        description: The end of the time range to fetch candlesticks for.
      - name: smooth
        located_in: Query
        type_name: boolean
        description: A flag that controls whether the candlestick is “smoothed” or not. A smoothed candlestick uses the previous candle’s close price as its open price, while an unsmoothed candlestick uses the first price from its time range as its open price. [default=False]
      - name: includeFirst
        located_in: Query
        type_name: boolean
        description: A flag that controls whether the candlestick that is covered by the from time should be included in the results. This flag enables clients to use the timestamp of the last completed candlestick received to poll for future candlesticks but avoid receiving the previous candlestick repeatedly. [default=True]
      - name: dailyAlignment
        located_in: Query
        type_name: integer
        description: The hour of the day (in the specified timezone) to use for granularities that have daily alignments. [default=17, minimum=0, maximum=23]
      - name: alignmentTimezone
        located_in: Query
        type_name: string
        description: The timezone to use for the dailyAlignment parameter. Candlesticks with daily alignment will be aligned to the dailyAlignment hour within the alignmentTimezone. Note that the returned times will still be represented in UTC. [default=America/New_York]
      - name: weeklyAlignment
        located_in: Query
        type_name: WeeklyAlignment
        description: The day of the week used for granularities that have weekly alignment. [default=Friday]
      - name: units
        located_in: Query
        type_name: DecimalNumber
        description: The number of units used to calculate the volume-weighted average bid and ask prices in the returned candles. [default=1]
      responses:
      - code: 200
        description: Pricing information has been successfully provided.
        headers:
        - name: RequestID
          description: The unique identifier generated for the request
        schema: !Struct
          fields:
          - name: instrument
            type_name: InstrumentName
            doc_string: The instrument whose Prices are represented by the candlesticks.
            is_array: false
            default: null
            required: false
          - name: granularity
            type_name: CandlestickGranularity
            doc_string: The granularity of the candlesticks provided.
            is_array: false
            default: null
            required: false
          - name: candles
            type_name: Candlestick
            doc_string: The list of candlesticks that satisfy the request.
            is_array: true
            default: null
            required: false
      other_responses:
      - 400
      - 401
      - 404
      - 405");
        let input: RestCall = annotate!(serde_yaml::from_str(input), "Parsing the input yaml")?;
        let dir = annotate!(tempdir(), "Creating temp dir")?;
        let code = super::create_endpoint(dir.path(), &[input])?;
        let code = code.to_string();
        println!("{code}");
        assert!(code.contains("pub fn list_all()"));
        Ok(())
    }
}
