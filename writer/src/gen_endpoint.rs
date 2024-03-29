//! Generates all the code for a single endpoint
mod gen_responses;

pub use self::gen_responses::gen_responses_for_call;
use crate::{
    util::{field_name, Location, ResponsesInfo, Writer},
    Error, Result,
};
use change_case::{lower_case, pascal_case, snake_case};
use error_stack::ResultExt;
use model::{
    endpoint_docs::{HttpMethod, Response, RestCall},
    Endpoint,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::{HashMap, HashSet};
use tracing::instrument;
use utils::pretty_doc_string;

/// Because the model crate doesn't have an error type, and we want
/// consistent methods that return errors on those types, we created
/// this local trait that can be applied to `model::RestCall`
pub trait CallNames {
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
    /// Returns the parts (from base_path) of the filename where the code for the response structs are stored
    /// eg. a filename for the `account` enpdoint and the `instruments` could be:
    /// format!("{base_path}/endpoints/account/responses/instruments.rs");
    /// .. In that case the reply to this call would be:
    /// ```rust
    /// vec!["endpoints", "account", "responses", "instruments"]
    ///     .into_iter()
    ///     .map(str::ToString);
    /// ```
    fn responses_module_parts(&self) -> Result<Vec<String>> {
        Ok(vec![
            "endpoints".to_string(),
            self.endpoint_name(),
            "responses".to_string(),
            self.method_name_as_string()?,
        ])
    }

    /// Returns the name of the endpoint that the Restcall is for
    fn endpoint_name(&self) -> String;

    /// All the responses possible
    /// The good response and a vec of all the bad responses
    fn good_and_bad_responses(&self) -> Result<(&Response, Vec<&Response>)>;

    /// The type name of the good response
    /// (All other responses will be turned into Errors)
    fn good_response_type_code(&self) -> Result<u16> {
        let (good_response, _bad_responses) = self.good_and_bad_responses()?;
        Ok(good_response.code)
    }
}

impl CallNames for RestCall {
    /// Returns the method_name without converting it to an Ident
    /// Generates a rust name for the method to trigger the Rest api call.
    ///
    /// Takes the path for the rest call. eg. `/v3/accounts/{accountID}/transactions`
    /// In most cases, just returns the last segment. eg. `transactions`
    /// In cases where the last segment is a variable. eg. `/v3/accounts/{accountid}/transactions/{transactionid}`
    /// The method name becomes the HTTP verb, eg `GET` or `POST`
    fn method_name_as_string(&self) -> Result<String> {
        // TODO: Remove this - it's in utils.rs now as `endpoint_method_name`
        // Get the last path part
        let (_, last_segment) = self.path.rsplit_once('/').ok_or_else(|| {
            Error::new(format!(
                "Couldn't make method_name out of path with no '/'s: {}",
                &self.path
            ))
        })?;
        let s = if last_segment.starts_with('{') {
            lower_case(&self.http_method.to_string())
        } else if self.http_method == HttpMethod::Post {
            format!("post_{last_segment}")
        } else {
            last_segment.to_string()
        };
        Ok(snake_case(s.as_str()))
    }

    /// All the responses possible
    /// The good response and a vec of all the bad responses
    fn good_and_bad_responses(&self) -> Result<(&Response, Vec<&Response>)> {
        let (good_responses, bad_responses): (Vec<&Response>, Vec<&Response>) = self
            .responses
            .iter()
            .partition(|r| (200..300).contains(&r.code));
        let Some(good_response) = good_responses.first() else {
            return Err(Error::new(format!(
                "Expected a single good response for {self:#?} but got zero"
            ))
            .into());
        };
        if good_responses.len() != 1 {
            return Err(Error::new(format!("Expected a single good response for {self:#?} but got all of these: {good_responses:#?}")).into());
        }
        Ok((good_response, bad_responses))
    }

    fn endpoint_name(&self) -> String {
        self.endpoint.to_string()
    }
}

pub trait ResponseNames {
    /// A centralized place to get the prefix for all the type names
    /// of the possible responses to a REST cal
    fn struct_prefix(call: &RestCall) -> Result<String> {
        call.response_struct_prefix()
    }
    /// Needs to be implemented to get the code
    fn code(&self) -> u16;
    /// Generates the type name for a certain response
    fn type_name(&self, prefix: &str) -> String {
        format!("{prefix}{code}", code = self.code())
    }
}

impl ResponseNames for Response {
    fn code(&self) -> u16 {
        self.code
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
    let with = snake_case(name);
    quote! {
        let url = url.replace(#to_replace, #with);
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
        HttpMethod::Get => quote! { self.client.get(self.client.start_get(url)).await? },
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

impl<'a> Writer<'a> {
    #[instrument(skip(response_map))]
    pub fn gen_endpoint_responses(
        &self,
        base_path: &str,
        endpoint_name: &str,
        response_map: &HashMap<String, ResponsesInfo>,
    ) -> Result<TokenStream> {
        // A map of call names to all the info about their responses
        // Used to generate the response structs, and then again to call them
        // Make the Response type for each call
        let modules = response_map
            .keys()
            .map(|key| Ident::new(key, Span::call_site()))
            .collect::<Vec<Ident>>();
        // Generate each of the responses sub-modules
        for ResponsesInfo {
            responses_module_parts,
            token_stream,
            ..
        } in response_map.values()
        {
            let filename = format!(
                "{base_path}/{parts}.rs",
                parts = responses_module_parts.join("/")
            );
            self.stream_to_file(token_stream.clone(), &filename)
                .attach_printable_lazy(|| format!("Saving endpoint to {filename}"))
                .change_context_lazy(Error::default)?;
        }
        // Generate the responses module itself
        Ok(quote!(
            #(pub mod #modules);*;
        ))
    }

    pub fn gen_endpoint(&self, endpoint: &Endpoint) -> Result<TokenStream> {
        let Endpoint { name, calls } = endpoint;
        let struct_name = pascal_case(name);
        let struct_ident = Ident::new(&struct_name, Span::call_site());
        let calls = calls
            .iter()
            .map(|call| gen_call(call, name))
            .collect::<Result<Vec<TokenStream>>>()?;

        let uses = self.get_uses(endpoint);

        Ok(quote!(
            use serde::{Serialize, Deserialize};

            pub mod responses;

            struct #struct_ident<'a> {
                client: &'a Client,
            }

            impl<'a> #struct_ident<'a> {
                #(#calls)*
            }
        ))
    }

    /// Generates the token_stream for all the uses clauses for all the types used in this endpoint
    fn get_uses(&self, endpoint: &Endpoint) -> TokenStream {
        let type_names = get_parameter_type_names(endpoint);
        let uses: HashSet<&Location<'_>> = type_names
            .iter()
            .flat_map(|tn| self.type_name_to_location(tn))
            .collect();
        let uses: Vec<TokenStream> = uses.iter().map(|l| l.as_uses()).collect();
        quote! {
            #(#uses)*
        }
    }
}

/// Given an `Endpoint` finds all the type names in the parameters that need to be imported
fn get_parameter_type_names(endpoint: &Endpoint) -> Vec<String> {
    let hash: HashSet<String> = endpoint
        .calls
        .iter()
        .flat_map(|call| call.parameters.iter())
        .map(|p| p.type_name.as_str())
        // Remove type_names that don't need to be imported
        .filter(|type_name| !["string", "List of"].contains(type_name))
        .map(pascal_case)
        .collect();
    hash.into_iter().collect()
}

#[cfg(test)]
mod unit_test {
    use crate::{
        gen_endpoint::{gen_responses::gen_response, ResponseNames},
        util::Writer,
        Error, Result,
    };
    use error_stack::ResultExt;
    use itertools::Itertools;
    use model::{
        definition_docs::{Field, Schema, Struct},
        endpoint_docs::{Response, ResponseHeader},
        Content, Endpoint,
    };
    use pretty_assertions::assert_eq;
    use utils::stream_to_string;

    #[test]
    fn test_gen_response() -> Result<()> {
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
        let prefix = "MyCall";
        let type_name = response.type_name(prefix);
        let ts = gen_response(&type_name, &response)?;
        let s = stream_to_string(&ts).change_context_lazy(Error::default)?;
        assert_eq!(
            s,
            r#"/// Pricing information has been successfully provided.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct MyCall200 {
    /// The latest candle sticks.
    latest_candles: Vec<CandlestickResponse>,
}
impl Default for MyCall200 {
    fn default() -> Self {
        Self {
            latest_candles: Default::default(),
        }
    }
}
"#
        );
        Ok(())
    }

    fn load_contents() -> Vec<Content> {
        let yaml = std::fs::read_to_string("../content.yaml").expect("Opening content.yaml");
        serde_yaml::from_str(&yaml).expect("Reading in content.yaml")
    }

    fn get_endpoint<'a>(contents: &'a [Content], endpoint_name: &str) -> &'a Endpoint {
        contents
            .iter()
            .flat_map(Content::as_endpoint)
            .find(|ep| ep.name == endpoint_name)
            .expect("Couldn't find account endpoint in parsed content")
    }

    #[test]
    fn test_get_parameter_type_names() {
        let contents = load_contents();
        let account_ep = get_endpoint(&contents, "account");
        let mut types = super::get_parameter_type_names(account_ep);
        let mut expected = vec!["TransactionID", "AcceptDatetimeFormat", "AccountID"];
        types.sort();
        expected.sort();
        assert_eq!(expected, types);
    }

    #[test]
    fn test_get_uses() {
        tracing_subscriber::fmt().try_init().ok();
        let contents = load_contents();
        let account_ep = get_endpoint(&contents, "account");

        let writer = Writer::new(&contents);
        let tokens = writer.get_uses(&account_ep);
        let as_text = stream_to_string(&tokens).unwrap();
        let expected = "use crate::definitions::account::AccountId;
use crate::definitions::transaction::TransactionId;
use crate::definitions::primitives::AcceptDatetimeFormat;";
        let got_lines: Vec<&str> = as_text.lines().sorted().collect();
        let expected_lines: Vec<&str> = expected.lines().sorted().collect();
        assert_eq!(expected_lines, got_lines);
    }
}
