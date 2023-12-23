use crate::error::{EasyError, Error, Result};
use crate::gen_client::gen_client;
use crate::gen_definition::gen_definition;
use crate::gen_endpoint::{gen_endpoint, gen_endpoint_responses};
use crate::state::State;
use error_stack::ResultExt;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use rust_format::{Config, Formatter, PrettyPlease};
use std::path::Path;

use crate::gen_error::gen_error;
use crate::gen_lib::gen_lib;
use model::{Content, Endpoint};

/// Writes a token_stream out to a file
pub fn stream_to_file(stream: TokenStream, path: &str) -> Result<()> {
    // Create the dir if it doesn't already exist
    let path = Path::new(path);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)
            .annotate_lazy(|| format!("Unable to create directory \"{dir:#?}\""))?;
    }

    let formatting_result = PrettyPlease::default()
        .format_tokens(stream.clone())
        .annotate_lazy(|| format!("Formatting code for {path:#?}"));
    let formatted_code = match formatting_result {
        Ok(code) => code,
        Err(err) => {
            tracing::error!("Unable to render token stream for {path:#?}. It has been rendered unformatted so you can inspect it: {err:#?}");
            format!("{stream}")
        }
    };
    std::fs::write(path, formatted_code)
        .annotate_lazy(|| format!("Unable to write to \"{path:#?}\""))?;
    Ok(())
}

/// Writes a token_stream into a string and makes it pretty if it can
pub fn stream_to_string(stream: &TokenStream) -> Result<String> {
    let config = Config::new_str().post_proc(rust_format::PostProcess::ReplaceMarkersAndDocBlocks);
    PrettyPlease::from_config(config)
        .format_tokens(stream.clone())
        .annotate_lazy(|| format!("Converting code to string {stream:#?}"))
}

/// Takes a raw doc string and returns a pretty token_stream
/// Example usage: `#(#doc_string)*`
pub fn pretty_doc_string(input: &str) -> Result<Vec<TokenStream>> {
    let lines = textwrap::wrap(input, 60);
    lines
        .iter()
        .map(|line| {
            let line = format!("/// {line}");
            line.parse()
                .map_err(|err| Error::Message(format!("{err:#?}")))
                .annotate_lazy(|| format!("While quoting docstring line: {line}"))
        })
        .collect::<Result<Vec<proc_macro2::TokenStream>>>()
        .attach_printable_lazy(|| format!("Trying to prettyize: {input}"))
}

/// Generate all the source code
pub fn generate_source(base_path: &str, contents: &[Content]) -> Result<()> {
    let mut mods = Vec::new();
    let state = State::default();
    // let mut endpoints = Vec::new();
    // Generate the error.rs
    mods.push("error");
    stream_to_file(gen_error(), &format!("{base_path}/error.rs"))
        .attach_printable("Writing error.rs")?;
    // Generate all the definitions we need
    for definition in contents.iter().flat_map(Content::definitions).flatten() {
        let content = gen_definition(definition)
            .attach_printable_lazy(|| format!("Generating definition for {}", definition.name))?;
        let tokens = quote! {
            use serde::{Serialize, Deserialize};

            #content
        };
        let filename = format!(
            "{base_path}/definitions/{}.rs",
            change_case::snake_case(&definition.name)
        );
        stream_to_file(tokens, &filename)
            .attach_printable_lazy(|| format!("Saving definition to {filename}"))?;
    }

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
        .attach_printable_lazy(|| format!("Saving endpoint to {filename}"))?;
    mods.push("endpoints");

    // Generate each of the endpoints
    for endpoint in contents.iter().flat_map(Content::as_endpoint) {
        let tokens = gen_endpoint(endpoint)
            .attach_printable_lazy(|| format!("Generating endpoint for {}", endpoint.name))?;
        let filename = format!("{base_path}/endpoints/{}.rs", endpoint.name);
        stream_to_file(tokens, &filename)
            .attach_printable_lazy(|| format!("Saving endpoint to {filename}"))?;
        // Generate the responses in a sub module
        let tokens = gen_endpoint_responses(base_path, endpoint)
            .attach_printable_lazy(|| format!("Generating endpoint for {}", endpoint.name))?;
        let filename = format!("{base_path}/endpoints/{}/responses.rs", endpoint.name);
        stream_to_file(tokens, &filename)
            .attach_printable_lazy(|| format!("Saving endpoint to {filename}"))?;
    }
    // Generate client.rs
    mods.push("client");
    stream_to_file(gen_client(&endpoints), &format!("{base_path}/client.rs"))
        .attach_printable("Writing client.rs")?;
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
    stream_to_file(gen_lib(mods.as_slice()), &format!("{base_path}/lib.rs"))
        .attach_printable("Generating lib.rs")?;
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
