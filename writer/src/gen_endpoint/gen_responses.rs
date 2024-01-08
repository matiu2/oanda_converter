use super::CallName;
use crate::{gen_definition::gen_struct, Error, Result};
use error_stack::ResultExt;
use model::{
    definition_docs::{Schema, Stream},
    endpoint_docs::{Response, RestCall},
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use tracing::{info, instrument};
use utils::pretty_doc_string;

/// Generates all the possible responses for a particular API call endpoint
#[instrument(skip(call))]
pub fn gen_responses_for_call(call: &RestCall) -> Result<TokenStream> {
    info!("Generating responses for: {}", &call.path);
    let struct_prefix = call.response_struct_prefix()?;
    let span = tracing::Span::current();
    // Get the good response (always 200 or 201)
    let (good_responses, bad_responses): (Vec<&Response>, Vec<&Response>) = call
        .responses
        .iter()
        .partition(|r| (200..300).contains(&r.code));
    let good_response = good_responses
        .first()
        .cloned()
        .map(|r| gen_response(&struct_prefix, r))
        .ok_or_else(|| Error::new("Expected at least one good response"))
        .attach_printable_lazy(|| format!("Generating good responses: {span:#?}"))??;
    let bad_responses = if !bad_responses.is_empty() {
        // Bad response names
        let bad_response_names: Vec<String> = bad_responses
            .iter()
            .map(|r| format!("{struct_prefix}{}", r.code))
            .collect();
        // Generate the structs
        let bad_response_structs = bad_response_names
            .iter()
            .zip(bad_responses.iter().cloned())
            .map(|(name, r)| gen_response_schema(&r.schema, name))
            .collect::<Result<Vec<TokenStream>>>()
            .attach_printable_lazy(|| format!("Generating bad responses: {span:#?}"))?;
        let structs = quote! { #(#bad_response_structs)* };

        // Generate the Error enum
        let variant_names = bad_response_names
            .iter()
            .zip(bad_responses.iter())
            .map(|(n, r)| {
                let code = format!("E{}", &r.code);
                let code = Ident::new(&code, Span::call_site());
                let struct_name = Ident::new(n, Span::call_site());
                quote! {#code(#struct_name)}
            });

        let error = quote! {
            #[derive(Debug)]
            pub enum Error {
                #(#variant_names),*
            }
        };

        quote! {
            use serde::{Serialize, Deserialize};

            #structs

            #error
        }
    } else {
        quote!()
    };

    // let enum_contents: Vec<TokenStream> = variant_names
    //     .zip(bad_responses.into_iter())
    //     .map(|(enum_name, contents)| {
    //         quote!(
    //             #enum_name(#contents)
    //         )
    //     })
    //     .collect();

    Ok(quote! {
        #good_response

        #bad_responses
    })
}

/// Generates the type that contents for response to ?
pub fn gen_response(struct_prefix: &str, response: &Response) -> Result<TokenStream> {
    let name = format!("{struct_prefix}{}", response.code);
    // let ident = Ident::new(&name, Span::call_site());
    let doc_string =
        pretty_doc_string(&response.description).change_context_lazy(Error::default)?;
    let schema = gen_response_schema(&response.schema, &name)?;
    // Make
    Ok(quote! {
        #(#doc_string)*
        #schema
    })
}

#[instrument(skip(schema))]
pub fn gen_response_schema(schema: &Schema, name: &str) -> Result<TokenStream> {
    info!("Generating response schema");
    match schema {
        Schema::Struct(r#struct) => gen_struct(r#struct, name),
        Schema::Stream(stream) => gen_response_stream(stream),
    }
}

fn gen_response_stream(_stream: &Stream) -> Result<TokenStream> {
    // TODO: Make this work
    Ok(quote!(
        struct Stream();
    ))
}
