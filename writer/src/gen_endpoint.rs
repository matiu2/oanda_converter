//! Generates all the code for a single endpoint
use model::Content;
use quote::{__private::TokenStream, quote};

pub fn gen_endpoint(endpoint: &Content) -> TokenStream {
    quote!()
}
