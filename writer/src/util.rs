use std::collections::HashMap;

use crate::error::Result;
// use crate::gen_client::gen_client;
use crate::gen_definition::gen_definition;
use crate::gen_endpoint::{
    gen_endpoint, gen_endpoint_responses, gen_responses_for_call, CallNames,
};
use crate::gen_mods::gen_mods;
use crate::Error;
use error_stack::ResultExt;
use model::endpoint_docs::{Response, RestCall};
use model::{Content, Endpoint};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use utils::stream_to_file;

/// Writes the Rest client definitions (json types) in rust
pub fn write_definitions(
    base_path: &str,
    mods: &mut Vec<&str>,
    contents: &[Content],
) -> Result<()> {
    // Generate all the definitions we need
    mods.push("definitions");
    let mut definition_mods = Vec::new();
    for definition in contents.iter().flat_map(Content::definitions).flatten() {
        let content = gen_definition(definition)
            .attach_printable_lazy(|| format!("Generating definition for {}", definition.name))?;
        let tokens = quote! {
            #content
        };
        let mod_name = change_case::snake_case(&definition.name);
        definition_mods.push(mod_name.clone());
        let filename = format!("{base_path}/definitions/{mod_name}.rs");
        stream_to_file(tokens, &filename)
            .change_context_lazy(|| Error::new(format!("Saving definition to {filename}")))?;
    }
    // Write definitions.rs
    stream_to_file(
        gen_mods(definition_mods.as_slice()),
        &format!("{base_path}/definitions.rs"),
    )
    .change_context_lazy(|| Error::new("Generating lib.rs"))?;
    Ok(())
}

/// Writes the source code for all the Rest API endpoints
/// Returns a list of endpoint names
pub fn write_endpoints<'a>(
    base_path: &'a str,
    mods: &mut Vec<&str>,
    contents: &'a [Content],
) -> Result<Vec<&'a str>> {
    // Just list the endpoint names
    let endpoints: Vec<&str> = contents
        .iter()
        .flat_map(Content::as_endpoint)
        .map(|Endpoint { name, .. }| name.as_str())
        .collect();

    // Generate endpoints.rs
    let tokens = gen_endpoints(&endpoints);
    let filename = format!("{base_path}/endpoints.rs");
    stream_to_file(tokens, &filename)
        .change_context_lazy(|| Error::new(format!("Saving endpoint to {filename}")))?;
    mods.push("endpoints");

    // Generate each of the endpoints
    for endpoint in contents.iter().flat_map(Content::as_endpoint) {
        let tokens = gen_endpoint(endpoint)
            .attach_printable_lazy(|| format!("Generating endpoint for {}", endpoint.name))?;
        let filename = format!("{base_path}/endpoints/{}.rs", endpoint.name);
        stream_to_file(tokens, &filename)
            .change_context_lazy(|| Error::new(format!("Saving endpoint to {filename}")))?;
        // Generate the responses in a sub module
        let responses_info = get_responses_info(&endpoint.calls)?;
        let tokens = gen_endpoint_responses(base_path, &endpoint.name, &responses_info)
            .attach_printable_lazy(|| format!("Generating endpoint for {}", endpoint.name))?;
        let filename = format!("{base_path}/endpoints/{}/responses.rs", endpoint.name);
        stream_to_file(tokens, &filename)
            .change_context_lazy(|| Error::new(format!("Saving endpoint to {filename}")))?;
    }

    Ok(endpoints)
}

pub struct ResponsesInfo<'a> {
    pub responses_module_parts: Vec<String>,
    pub struct_prefix: String,
    pub good_response: &'a Response,
    pub bad_responses: Vec<&'a Response>,
    pub token_stream: TokenStream,
}

fn get_responses_info(calls: &[RestCall]) -> Result<HashMap<String, ResponsesInfo>> {
    calls
        .iter()
        .map(|call| {
            let struct_prefix = call.response_struct_prefix()?;
            let responses_module_parts = call.responses_module_parts()?;
            let (good_response, bad_responses) = call.good_and_bad_responses()?;
            let token_stream =
                gen_responses_for_call(&struct_prefix, good_response, bad_responses.as_slice())?;
            Ok((
                call.method_name_as_string()?,
                ResponsesInfo {
                    responses_module_parts,
                    struct_prefix,
                    good_response,
                    bad_responses,
                    token_stream,
                },
            ))
        })
        .collect::<Result<HashMap<String, ResponsesInfo>>>()
}

/// Generate all the source code
pub fn generate_source(base_path: &str, contents: &[Content]) -> Result<()> {
    let mut mods = vec!["host", "error", "client"];

    write_definitions(base_path, &mut mods, contents)?;
    write_endpoints(base_path, &mut mods, contents)?;

    // We use the mods here
    let mods = gen_mods(mods.as_slice());
    let lib = quote! {
        #mods

        pub use error::{Error, Result};
    };
    stream_to_file(lib, &format!("{base_path}/lib.rs"))
        .change_context_lazy(|| Error::new("Generating lib.rs"))?;
    Ok(())
}

/// Just generates the src/endpoints.rs
fn gen_endpoints(endpoints: &[&str]) -> TokenStream {
    let uses = endpoints.iter().map(|ep| {
        let ep = Ident::new(ep, proc_macro2::Span::call_site());
        quote!(pub mod #ep;)
    });
    quote!(#(#uses)*)
}

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => { return Err(error_stack::Report::new($crate::error::Error::Message(format!($($arg),*)))) };
}

/// Takes a name from the yaml and turns it into a nice field name
pub fn field_name(name: &str) -> TokenStream {
    if name == "type" {
        let name = format_ident! { "r#type" };
        quote! { #name }
    } else {
        let name = name.replace('-', "_");
        let name = change_case::snake_case(&name);
        let name = Ident::new(&name, proc_macro2::Span::call_site());
        quote! { #name }
    }
}
