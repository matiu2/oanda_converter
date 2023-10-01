use error_stack::ResultExt;
use model::Content;
use util::generate_source;

use crate::error::{EasyError, Result};
mod error;
mod gen_client;
mod gen_definition;
mod gen_endpoint;
mod gen_error;
mod gen_lib;
mod util;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // List of rust modules to declare
    let mut mods = vec!["error"];
    let base_path = "oanda_v2";
    // You will have already run 'serialize_all' and generated a content.yaml. Now we'll read it in
    let yaml = std::fs::read_to_string("content.yaml").annotate("Opening content.yaml")?;
    let content: Vec<Content> = serde_yaml::from_str(&yaml).annotate("Reading in content.yaml")?;
    generate_source(base_path, content.as_slice()).attach_printable("Generating the source")?;
    Ok(())
}
