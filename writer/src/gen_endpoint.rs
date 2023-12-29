//! Generates all the code for a single endpoint
mod gen_responses;

use self::gen_responses::gen_responses_for_call;
use crate::{util::field_name, Error, Result};
use change_case::{lower_case, pascal_case, snake_case};
use error_stack::ResultExt;
use model::{
    endpoint_docs::{HttpMethod, RestCall},
    Endpoint,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use tracing::{info, instrument};
use utils::{pretty_doc_string, stream_to_file};

pub trait CallName {
    /// Basic method name - don't convert it to an Ident
    fn method_name_as_string(&self) -> Result<String>;
    /// Given self as a RestCall returns the method_name you should use
    fn method_name(&self) -> Result<Ident> {
        self.method_name_as_string()
            .map(|basic| Ident::new(basic.as_str(), Span::call_site()))
    }
    /// Name/prefix for the response struct
    fn response_struct_prefix(&self) -> Result<String> {
        Ok(pascal_case(&self.method_name_as_string()?))
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
    let value = field_name(name);
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

/// Generates a single method that performs a Rest API call for a certain endpoint
fn gen_call(call: &RestCall, endpoint_name: &str) -> Result<TokenStream> {
    let RestCall { path, .. } = call;
    let method_name = call
        .method_name()
        .attach_printable_lazy(|| format!("for endpoint {endpoint_name}"))?;
    let doc_string = pretty_doc_string(&call.doc_string).change_context_lazy(Error::default)?;
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
    Ok(quote!(
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
            let name = field_name(&p.name);
            let type_name = Ident::new(&pascal_case(&p.type_name), Span::call_site());
            quote! { #name: #type_name }
        })
        .collect();
    Ok(quote! {#(#params),*})
}

#[instrument(skip(calls))]
pub fn gen_endpoint_responses(
    base_path: &str,
    Endpoint {
        name: endpoint_name,
        calls,
    }: &Endpoint,
) -> Result<TokenStream> {
    info!("Generating responses for endpoint");
    // Make the Response type for each call
    let response_map = calls
        .iter()
        .map(|call| Ok((call.method_name_as_string()?, gen_responses_for_call(call)?)))
        .collect::<Result<HashMap<String, TokenStream>>>()?;
    let modules = response_map
        .keys()
        .map(|key| Ident::new(key.as_str(), Span::call_site()))
        .collect::<Vec<Ident>>();
    // Generate each of the responses sub-modules
    for (name, tokens) in response_map {
        let filename = format!("{base_path}/endpoints/{endpoint_name}/responses/{name}.rs");
        stream_to_file(tokens, &filename)
            .attach_printable_lazy(|| format!("Saving endpoint to {filename}"))
            .change_context_lazy(Error::default)?;
    }
    // Generate the responses module itself
    Ok(quote!(
        #(pub mod #modules);*;
    ))
}

pub fn gen_endpoint(endpoint: &Endpoint) -> Result<TokenStream> {
    let Endpoint { name, calls } = endpoint;
    let struct_name = pascal_case(name);
    let struct_ident = Ident::new(&struct_name, Span::call_site());
    let calls = calls
        .iter()
        .map(|call| gen_call(call, name))
        .collect::<Result<Vec<TokenStream>>>()?;

    Ok(quote!(
        use crate::{client::Client, Error, Result};

        pub mod responses;

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
    use crate::{gen_endpoint::gen_responses::gen_response, Error, Result};
    use error_stack::ResultExt;
    use model::{
        definition_docs::{Field, Schema, Struct},
        endpoint_docs::{Response, ResponseHeader},
    };
    use pretty_assertions::assert_eq;
    use utils::stream_to_string;

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
        let struct_prefix = "MyCall";
        let ts = gen_response(struct_prefix, &response)?;
        let s = stream_to_string(&ts).change_context_lazy(Error::default)?;
        assert_eq!(
            s,
            r#"/// Pricing information has been successfully provided.
#[derive(Serialize, Deserialize)]
struct MyCall200 {
    /// The latest candle sticks.
    latest_candles: Vec<CandlestickResponse>,
}
"#
        );
        Ok(())
    }
}
