//! Generate the lib.rs in `oanda_v2`

use quote::{
    __private::{Span, TokenStream},
    quote,
};

/// Generates the lib.rs in our output
///
/// ## Parameters
///
/// * `mods` - List of mod lines to create
pub fn gen_lib(mods: &[&str]) -> TokenStream {
    mods.iter()
        .map(|module| syn::Ident::new(module, Span::call_site()))
        .map(|module| quote!(pub mod #module;))
        .collect()
}
