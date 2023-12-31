//! Generates a TokenStream that just declares a bunch of `pubm mod xxx`

use proc_macro2::{Span, TokenStream};
use quote::quote;

/// Turns a list of mod names into a TokenStream generating a `pub mod` for each one.
///
/// ## Parameters
///
/// * `mods` - List of mod names to generate code for
pub fn gen_mods(mods: &[impl ToString]) -> TokenStream {
    let generated: TokenStream = mods
        .iter()
        .map(|module| syn::Ident::new(&module.to_string(), Span::call_site()))
        .map(|module| quote!(pub mod #module;))
        .collect();
    quote! {
        #generated
    }
}
