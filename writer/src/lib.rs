pub mod error;
mod gen_client;
pub mod gen_definition;
mod gen_endpoint;
mod gen_error;
mod gen_mods;
pub mod util;

pub use error::{EasyError, Error, Result};
