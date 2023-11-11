//! Generates all the code for a single endpoint
use crate::{util::pretty_doc_string, Error, Result};
use change_case::{lower_case, pascal_case, snake_case};
use error_stack::ResultExt;
use model::{
    definition_docs::{Schema, Stream},
    endpoint_docs::{HttpMethod, Response, RestCall},
    Endpoint,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use tracing::instrument;

pub trait CallName {
    /// Basic method name - don't convert it to an Ident
    fn method_name_as_string(&self) -> Result<String>;
    /// Given self as a RestCall returns the method_name you should use
    fn method_name(&self) -> Result<Ident> {
        self.method_name_as_string()
            .map(|basic| Ident::new(basic.as_str(), Span::call_site()))
    }
    /// Name/prefix for the response struct
    fn response_struct_name(&self) -> Result<String> {
        let method_name = pascal_case(&self.method_name_as_string()?);
        Ok(format!("{method_name}_response"))
    }
}

impl CallName for RestCall {
    /// Returns the method_name without converting it to an Ident
    /// Generates a rust name for the method to trigger the Rest api call.
    ///
    /// Takes the path for the rest call. eg. `/v3/accounts/{accountID}/transactions`
    /// In most cases, just returns the last segment. eg. `transactions`
    /// In cases where the last segment is a variable. eg. `/v3/accounts/{accountid}/transactions/{transactionid}`
    /// The method name becomes the HTTP verb, eg `GET` or `POST`
    fn method_name_as_string(&self) -> Result<String> {
        // Get the last path part
        let (_, last_segment) = self.path.rsplit_once('/').ok_or_else(|| {
            Error::new(format!(
                "Couldn't make method_name out of path with no '/'s: {}",
                &self.path
            ))
        })?;
        let s = if last_segment.starts_with('{') {
            lower_case(&self.http_method.to_string())
        } else {
            last_segment.to_string()
        };
        Ok(snake_case(s.as_str()))
    }
}

/// Generate the code where we're inserting parameters into the url
fn gen_path_params(call: &RestCall) -> TokenStream {
    call.parameters
        .iter()
        .filter(|p| p.located_in.is_path())
        .map(|p| gen_path_param(&p.name))
        .collect()
}

/// Generate the code where we're passing parameter to the rest API
fn gen_header_params(call: &RestCall) -> Result<TokenStream> {
    call.parameters
        .iter()
        .filter(|p| p.located_in.is_header())
        .map(|p| gen_header_param(&p.name, &p.description))
        .collect()
}

/// Generate the code where we're passing parameter to the rest API
fn gen_query_params(call: &RestCall) -> Result<TokenStream> {
    let params = call
        .parameters
        .iter()
        .filter(|p| p.located_in.is_query())
        .map(|p| gen_query_param(&p.name))
        .collect::<Result<Vec<TokenStream>>>()?;
    Ok(quote! { [#(#params),*] })
}

/// Generates code that passes a parameter via an http get param
fn gen_query_param(name: &str) -> Result<TokenStream> {
    let value = Ident::new(&snake_case(name), Span::call_site());
    Ok(quote! { (#name, #value) })
}

/// Generates code that passes a parameter in the path through reqwest
fn gen_path_param(name: &str) -> TokenStream {
    let to_replace = quote! {"{" + #name + "}"};
    quote! {
        let url = url.replace(#to_replace);
    }
}

fn gen_header_param(header_name: &str, description: &str) -> Result<TokenStream> {
    let comment: TokenStream = format!("// {description}")
        .parse()
        .map_err(|err| Error::new(format!("Couldn't turn this comment into tokens: Error '{err:#?}' -- comment contents: {description}")))?;
    let value = Ident::new(&snake_case(header_name), Span::call_site());
    Ok(quote! {
        #comment
        .header(
        #header_name,
        #value
    )})
}

/// Generates all the possible responses for a particular API call endpoint
#[instrument(skip(call))]
fn gen_responses(struct_prefix: &str, call: &RestCall) -> Result<TokenStream> {
    let span = tracing::Span::current();
    let method_name = call.method_name()?;
    // Generate a struct for the Ok response
    let ok_name = format!("{struct_prefix}Ok");
    let ok_ident = Ident::new(&ok_name, Span::call_site());
    // TODO: Make a response for every code. Make a struct for the OK code and an enum for all the error codes

    // We generate a type for every call, because there are very few repeat types
    // $ open content.yaml |
    //   get documentation |
    //   select name calls? |
    //   where calls != null |
    //   select name calls |
    //   flatten |
    //   flatten |
    //   select name responses |
    //   flatten |
    //   flatten |
    //   where code != 200 |
    //   select name code schema.fields |
    //    select name code schema_fields.name  schema_fields.type_name  |
    //   upsert pair { get schema_fields_name schema_fields_type_name |
    //   str join " - " } |
    //   select pair  |
    //   uniq -c |
    //   sort-by count -nr
    // Generate an enum for the error
    let enum_name = format!("{struct_prefix}{method_name}Response");
    let enum_ident = Ident::new(&enum_name, Span::call_site());
    // Get the good response (always 200 or 201)
    let (good_responses, bad_responses): (Vec<&Response>, Vec<&Response>) = call
        .responses
        .iter()
        .partition(|r| (200..300).contains(&r.code));
    let good_response = good_responses
        .first()
        .cloned()
        .map(gen_response)
        .ok_or_else(|| Error::new("Expected at least one good response"))
        .attach_printable_lazy(|| format!("Generating good responses: {span:#?}"))??;
    // We'll put the bad responses in their own error enum

    let variant_names = bad_responses
        .iter()
        .map(|r| Ident::new(&format!("E{}", r.code), Span::call_site()));
    let bad_responses = bad_responses
        .iter()
        .cloned()
        .map(gen_response)
        .collect::<Result<TokenStream>>()
        .attach_printable_lazy(|| format!("Generating bad responses: {span:#?}"))?;

    let enum_contents: Vec<TokenStream> = variant_names
        .zip(bad_responses.into_iter())
        .map(|(enum_name, contents)| {
            quote!(
                #enum_name: #contents
            )
        })
        .collect();

    Ok(quote! {
        #good_response

        pub enum #enum_ident {
            #(#enum_contents),*
        }
    })
}

/// Generates the type that contents for response to ?
fn gen_response(response: &Response) -> Result<TokenStream> {
    let name = format!("Code{}", response.code);
    // let ident = Ident::new(&name, Span::call_site());
    let doc_string = pretty_doc_string(&response.description)?;
    let schema = gen_response_schema(&response.schema, &name)?;
    // Make
    Ok(quote! {
        #(#doc_string)*
        #schema
    })
}

pub fn gen_response_schema(schema: &Schema, name: &str) -> Result<TokenStream> {
    match schema {
        Schema::Struct(r#struct) => crate::gen_definition::gen_struct(r#struct, name),
        Schema::Stream(stream) => gen_response_stream(stream),
    }
}

fn gen_response_stream(stream: &Stream) -> Result<TokenStream> {
    todo!()
}

/// Generates a single method that performs a Rest API call for a certain endpoint
fn gen_call(call: &RestCall, endpoint_name: &str) -> Result<TokenStream> {
    let RestCall { path, .. } = call;
    let method_name = call
        .method_name()
        .attach_printable_lazy(|| format!("for endpoint {endpoint_name}"))?;
    let doc_string = pretty_doc_string(&call.doc_string)?;
    let http_method = match call.http_method {
        HttpMethod::Get => quote! { self.client.get(url) },
        HttpMethod::Post => quote! { self.client.post(url) },
        HttpMethod::Put => quote! { self.client.put(url) },
        HttpMethod::Patch => quote! { self.client.patch(url) },
    };
    let param_inputs = gen_params(call)?;
    let path_params = gen_path_params(call);
    let query_params = gen_query_params(call)?;
    let header_params = gen_header_params(call)?;
    let struct_prefix = call.response_struct_name()?;
    let responses = gen_responses(&struct_prefix, call)?;
    Ok(quote!(
        #responses

        #(#doc_string)*
        pub async fn #method_name(&self, #param_inputs) -> Result<()> {
            let url = #path;
            #path_params
            let url = self.client.url(url);
            let query = #query_params;
            let response = #http_method
            #header_params
            .query(&query)
            .send()
            .await?;
            let status_code = response.status_code();
            // TODO: Convert the response into the right thing
        }
    ))
}

/// Just a comma delimited list of params
fn gen_params(call: &RestCall) -> Result<TokenStream> {
    let params: Vec<TokenStream> = call
        .parameters
        .iter()
        .map(|p| {
            let name = Ident::new(&snake_case(&p.name), Span::call_site());
            let type_name = Ident::new(&pascal_case(&p.type_name), Span::call_site());
            quote! { #name: #type_name }
        })
        .collect();
    Ok(quote! {#(#params),*})
}

pub fn gen_endpoint(endpoint: &Endpoint) -> Result<TokenStream> {
    let Endpoint { name, calls } = endpoint;
    let struct_name = pascal_case(name);
    let struct_ident = Ident::new(&struct_name, Span::call_site());
    // Make the Response type for each call
    let calls = calls
        .iter()
        .map(|call| gen_call(call, name))
        .collect::<Result<Vec<TokenStream>>>()?;

    Ok(quote!(
        use crate::client::Client;

        struct #struct_ident<'a> {
            client: &'a Client,
        }

        impl<'a> #struct_ident<'a> {
            #(#calls)*
        }
    ))
}

#[cfg(test)]
mod unit_test {
    use crate::{util::stream_to_string, Result};
    use model::{
        definition_docs::{Field, Schema, Struct},
        endpoint_docs::{Response, ResponseHeader},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_gen_test_response() -> Result<()> {
        let response = Response {
            code: 200,
            description: "Pricing information has been successfully provided.".to_string(),
            headers: vec![ResponseHeader {
                name: "RequestID".to_string(),
                description: "The unique identifier generated for the request".to_string(),
            }],
            schema: Schema::Struct(Struct {
                fields: vec![Field {
                    name: "latestCandles".to_string(),
                    type_name: "CandlestickResponse".to_string(),
                    doc_string: "The latest candle sticks.".to_string(),
                    is_array: true,
                    r#default: None,
                    required: false,
                }],
            }),
        };
        let ts = super::gen_response(&response)?;
        let s = stream_to_string(&ts)?;
        assert_eq!(
            s,
            r#"/// Pricing information has been successfully provided.
#[derive(Serialize, Deserialize)]
struct Code200 {
    /// The latest candle sticks.
    latest_candles: Vec<CandlestickResponse>,
}
"#
        );
        Ok(())
    }
}
