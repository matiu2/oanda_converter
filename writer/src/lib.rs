use error::{IntoReport, Result};
use error_stack::ResultExt;
use quote::__private::TokenStream;
use rust_format::{Config, Formatter, PrettyPlease};
use std::fs;

pub mod error;
mod gen_error;
mod gen_lib;

use gen_error::gen_error;
use gen_lib::gen_lib;

/// Writes a token_stream out to a file
fn stream_to_file(stream: TokenStream, path: &str) -> error::Result<()> {
    let config = Config::new_str().post_proc(rust_format::PostProcess::ReplaceMarkersAndDocBlocks);
    let formatted_code = PrettyPlease::from_config(config)
        .format_tokens(stream.clone())
        .annotate_lazy(|| format!("Formatting code for {path}: {stream:#?}"))?;
    fs::write(path, formatted_code).annotate("Unable to write file")?;
    Ok(())
}

/// Generate all the source code
pub fn generate_source(base_path: &str) -> Result<()> {
    let mut mods = Vec::new();
    // Generate the error.rs
    mods.push("error");
    stream_to_file(gen_error(), &format!("{base_path}/error.rs"))
        .attach_printable("Writing error.rs")?;
    stream_to_file(gen_lib(mods.as_slice()), &format!("{base_path}/lib.rs"))
        .attach_printable("Generating lib.rs")?;
    Ok(())
}
