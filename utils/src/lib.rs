mod error;
pub use crate::error::Error;
use crate::error::{EasyError, Result};
use error_stack::ResultExt;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use rust_format::{Config, Formatter, PrettyPlease};
use std::path::Path;

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

/// Make a comment pretty. Wraps to 60 chars width
///
/// ## Arguments
///
/// * `input`: The comment text
/// * `prefix`: The prefix, eg. `//` or `///`
pub fn pretty_comment_basic(input: &str, prefix: &str) -> Result<Vec<TokenStream>> {
    let lines = textwrap::wrap(input, 60);
    lines
        .iter()
        .map(|line| {
            let line = format!("{prefix} {line}");
            line.parse()
                .map_err(|err| Error::Message(format!("{err:#?}")))
                .annotate_lazy(|| format!("While quoting docstring line: {line}"))
        })
        .collect::<Result<Vec<proc_macro2::TokenStream>>>()
        .attach_printable_lazy(|| format!("Trying to prettyize: {input}"))
}

/// Takes a raw doc string and returns a pretty token_stream
/// Example usage: `#(#doc_string)*`
pub fn pretty_doc_string(input: &str) -> Result<Vec<TokenStream>> {
    pretty_comment_basic(input, "///")
}

/// Takes a raw comment text, and returns it as a vec of token streams
/// It wraps them to 60 character widths
pub fn pretty_comment(input: &str) -> Result<Vec<TokenStream>> {
    pretty_comment_basic(input, "//")
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
