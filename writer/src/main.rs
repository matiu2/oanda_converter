use model::Content;
use proc_macro2::TokenStream;
use rust_format::{Formatter, PrettyPlease};

use crate::error::{EasyError, Result};
mod error;
mod gen_error;
mod gen_lib;

/// Writes a token_stream out to a file
fn stream_to_file(stream: TokenStream, path: &str) -> Result<()> {
    let formatted_code = PrettyPlease::default()
        .format_tokens(stream)
        .annotate_lazy(|| format!("Formatting code for {path}"))?;
    std::fs::write(path, formatted_code)
        .annotate_lazy(|| format!("Unable to write to \"{path}\""))?;
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // List of rust modules to declare
    let mut mods = vec!["error"];
    let base_path = "oanda_v2";
    // You will have already run 'serialize_all' and generated a content.yaml. Now we'll read it in
    let yaml = std::fs::read_to_string("content.yaml").annotate("Opening content.yaml")?;
    let content: Content = serde_yaml::from_str(&yaml).annotate("Reading in content.yaml")?;

    // generate_source(base_path, )
    Ok(())
}
