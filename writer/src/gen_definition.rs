//! Generates error.rs for oanda_v2
use model::Definition;
use quote::{__private::TokenStream, quote};

pub fn gen_definition(definition: &Definition) -> TokenStream {
    match definition.value {}
}
