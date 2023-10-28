//! Generates all the code for a single endpoint
use crate::{util::pretty_doc_string, Error, Result};
use change_case::{lower_case, pascal_case, snake_case};
use error_stack::ResultExt;
use model::{
    definition_docs::{Schema, Stream, Struct},
    endpoint_docs::{HttpMethod, LocatedIn, Response, RestCall, RestCallParameter},
    Endpoint,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub trait CallName {
    /// Basic method name - don't convert it to an Ident
    fn method_name_as_string(&self) -> Result<String>;
    /// Given self as a RestCall returns the method_name you should use
    fn method_name(&self) -> Result<Ident> {
        self.method_name_as_string()
            .map(|basic| Ident::new(basic.as_str(), Span::call_site()))
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

/// Generate the code where we're passing parameter to the rest API
fn gen_param_passings(call: &RestCall) -> Result<TokenStream> {
    call.parameters.iter().map(gen_param_pass).collect()
}

/// Generate the code that passes a parameter to the rest call
fn gen_param_pass(param: &RestCallParameter) -> Result<TokenStream> {
    match param.located_in {
        LocatedIn::Header => gen_header_param(&param.name, &param.description),
        LocatedIn::Path => todo!(),
        LocatedIn::Query => todo!(),
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

fn gen_responses(struct_name: &str, call: &RestCall) -> Result<TokenStream> {
    let method_name = call.method_name()?;
    let enum_name = format!("{struct_name}{method_name}Response");
    let enum_ident = Ident::new(&enum_name, Span::call_site());
    let responses = call
        .responses
        .iter()
        .map(gen_response)
        .collect::<Result<TokenStream>>()?;
    Ok(quote! {
        pub enum #enum_ident {
            #responses
        }
    })
}

/// Generates the type that A
fn gen_response(response: &Response) -> Result<TokenStream> {
    let name = format!("Code{}", response.code);
    let ident = Ident::new(&name, Span::call_site());
    let doc_string = pretty_doc_string(&response.description)?;
    let schema = gen_response_schema(&response.schema);
    // Make xd
    Ok(quote! {
        #(#doc_string)*
        #ident

    })
}

pub fn gen_response_schema(schema: &Schema) -> Result<TokenStream> {
    match schema {
        Schema::Struct(r#struct) => gen_response_struct(r#struct),
        Schema::Stream(stream) => gen_response_stream(stream),
    }
}

fn gen_response_stream(stream: &Stream) -> Result<TokenStream> {
    todo!()
}

fn gen_response_struct(r#struct: &Struct) -> Result<TokenStream> {
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
    let params = gen_params(call)?;
    let param_usage = gen_param_passings(call)?;
    Ok(quote!(
        #(#doc_string)*
        pub async fn #method_name(&self, #params) -> Result<()> {
            let url = self.client.url(#path);
            #http_method
            #param_usage;
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
