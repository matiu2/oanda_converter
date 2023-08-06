use error::{Error, IntoReport, Result};
use error_stack::ResultExt;
use quote::__private::TokenStream;
use rust_format::{Config, Formatter, PrettyPlease};
use std::fs;

pub mod error;
mod gen_client;
mod gen_definition;
mod gen_endpoint;
mod gen_error;
mod gen_lib;

use gen_error::gen_error;
use gen_lib::gen_lib;
use model::Content;

/// Writes a token_stream out to a file
fn stream_to_file(stream: TokenStream, path: &str) -> Result<()> {
    let formatted_code = stream_to_string(stream)?;
    fs::write(path, formatted_code).annotate_lazy(|| format!("Unable to write file: {path}"))?;
    Ok(())
}

/// Writes a token_stream out to a file
fn stream_to_string(stream: TokenStream) -> Result<String> {
    let config = Config::new_str().post_proc(rust_format::PostProcess::ReplaceMarkersAndDocBlocks);
    PrettyPlease::from_config(config)
        .format_tokens(stream.clone())
        .annotate_lazy(|| format!("Formatting code to string {stream:#?}"))
}

/// Takes a raw doc string and returns a pretty token_stream
/// Example usage: `#(#doc_string)*`
fn pretty_doc_string(input: &str) -> Result<Vec<TokenStream>> {
    input
        .lines()
        .map(|line| {
            let line = format!("/// {line}");
            line.parse()
                .map_err(|err| Error::Message(format!("{err:#?}")))
                .annotate_lazy(|| "While quoting docstring line: {line}")
        })
        .collect::<Result<Vec<proc_macro2::TokenStream>>>()
}

/// Generate all the source code
pub fn generate_source(base_path: &str, contents: &[Content]) -> Result<()> {
    let mut mods = Vec::new();
    // let mut endpoints = Vec::new();
    // Generate the error.rs
    mods.push("error");
    stream_to_file(gen_error(), &format!("{base_path}/error.rs"))
        .attach_printable("Writing error.rs")?;
    mods.push("client");
    // Generate all the definitions we need
    // for content in contents {
    //     if let Some(definitions) = content.definitions() {}
    // }
    // Generate each of the endpoints
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
