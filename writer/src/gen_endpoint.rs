//! Generates all the code for a single endpoint
use crate::{util::pretty_doc_string, Error, Result};
use change_case::{lower_case, pascal_case};
use error_stack::ResultExt;
use model::{
    definition_docs::{Schema, Stream, Struct},
    endpoint_docs::{Response, RestCall},
    Endpoint,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub trait CallName {
    /// Given self as a RestCall returns the method_name you should use
    fn method_name(&self) -> Result<Ident>;
}

impl CallName for RestCall {
    /// Generates a rust name for the method to trigger the Rest api call.
    ///
    /// Takes the path for the rest call. eg. `/v3/accounts/{accountID}/transactions`
    /// In most cases, just returns the last segment. eg. `transactions`
    /// In cases where the last segment is a variable. eg. `/v3/accounts/{accountid}/transactions/{transactionid}`
    /// The method name becomes the HTTP verb, eg `GET` or `POST`
    fn method_name(&self) -> Result<Ident> {
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
        Ok(Ident::new(
            &pascal_case(s.as_str()),
            proc_macro2::Span::call_site(),
        ))
    }
}

fn gen_responses(struct_name: &str, call: &RestCall) -> Result<TokenStream> {
    let method_name = call.method_name()?;
    let enum_name = format!("{struct_name}{method_name}Response");
    let enum_ident = Ident::new(&enum_name, proc_macro2::Span::call_site());
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

fn gen_response(response: &Response) -> Result<TokenStream> {
    let name = format!("Code{}", response.code);
    let ident = Ident::new(&name, proc_macro2::Span::call_site());
    let doc_string = pretty_doc_string(&response.description)?;
    let schema = gen_response_schema(&response.schema);
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

pub fn gen_endpoint(endpoint: &Endpoint) -> Result<TokenStream> {
    let Endpoint { name, calls } = endpoint;
    let struct_name = pascal_case(name);
    let struct_ident = Ident::new(&struct_name, proc_macro2::Span::call_site());
    // Make the Response type for each call
    let calls = calls
        .iter()
        .map(|call| {
            let method_name = call
                .method_name()
                .attach_printable_lazy(|| format!("for endpoint {name}"))?;
            Ok(quote!(
                pub async fn #method_name(&self) -> Result<()> {
                    todo!()
                }
            ))
        })
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
