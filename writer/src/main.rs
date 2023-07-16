use error::{IntoReport, Result};
use error_stack::ResultExt;
use quote::__private::TokenStream;
use rust_format::{Formatter, PrettyPlease};
use std::fs;

pub mod error;
mod gen_error;
mod gen_lib;

use gen_error::gen_error;
use gen_lib::gen_lib;

/// Writes a token_stream out to a file
fn stream_to_file(stream: TokenStream, path: &str) -> error::Result<()> {
    let formatted_code = PrettyPlease::default()
        .format_tokens(stream)
        .annotate_lazy(|| format!("Formatting code for {path}"))?;
    fs::write(path, formatted_code).annotate("Unable to write file")?;
    Ok(())
}

fn main() -> Result<()> {
    let mut mods = Vec::new();
    // Generate the error.rs
    mods.push("error");
    stream_to_file(gen_error(), "error.rs").attach_printable("Writing error.rs")?;
    stream_to_file(gen_lib(mods.as_slice()), "lib.rs").attach_printable("Generating lib.rs")?;
    Ok(())
}
