//! Generate the lib.rs in `oanda_v2`

use proc_macro2::{Span, TokenStream};
use quote::quote;

/// Generates the lib.rs in our output
///
/// ## Parameters
///
/// * `mods` - List of mod lines to create
pub fn gen_lib(mods: &[&str]) -> TokenStream {
    let generated: TokenStream = mods
        .iter()
        .map(|module| syn::Ident::new(module, Span::call_site()))
        .map(|module| quote!(pub mod #module;))
        .collect();
    quote! {
        pub mod host;
        #generated

        pub use error::{Error, Result};
    }
}
