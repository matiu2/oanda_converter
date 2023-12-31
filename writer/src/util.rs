use crate::error::Result;
use crate::gen_client::gen_client;
use crate::gen_definition::gen_definition;
use crate::gen_endpoint::{gen_endpoint, gen_endpoint_responses};
use crate::gen_error::gen_error;
use crate::gen_mods::gen_mods;
use crate::Error;
use error_stack::ResultExt;
use model::{Content, Endpoint};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use utils::stream_to_file;

/// Generate all the source code
pub fn generate_source(base_path: &str, contents: &[Content]) -> Result<()> {
    let mut mods = vec!["host"];
    // let mut endpoints = Vec::new();
    // Generate the error.rs
    mods.push("error");
    stream_to_file(gen_error(), &format!("{base_path}/error.rs"))
        .change_context(Error::new("Writing error.rs"))?;
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
        let tokens = gen_endpoint_responses(base_path, endpoint)
            .attach_printable_lazy(|| format!("Generating endpoint for {}", endpoint.name))?;
        let filename = format!("{base_path}/endpoints/{}/responses.rs", endpoint.name);
        stream_to_file(tokens, &filename)
            .change_context_lazy(|| Error::new(format!("Saving endpoint to {filename}")))?;
    }
    // Generate client.rs
    mods.push("client");
    stream_to_file(gen_client(&endpoints), &format!("{base_path}/client.rs"))
        .change_context_lazy(|| Error::new("Writing client.rs"))?;
    // for endpoint in contents {
    //     stream_to_file(
    //         gen_endpoint::gen_endpoint(&endpoint),
    //         &format!("{base_path}/{}.rs", &endpoint.name()),
    //     )
    //     .attach_printable_lazy(|| format!("Endpoint {}", &endpoint.name()))?;
    // }
    // // We use the endpoints here
    // stream_to_file(gen_client::gen_client(), &format!("{base_path}/client.rs"))
    //     .attach_printable("Generating client.rs")?;
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
        quote!(mod #ep;)
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
