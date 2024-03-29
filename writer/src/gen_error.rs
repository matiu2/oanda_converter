//! Generates error.rs for oanda_v2
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_error() -> TokenStream {
    quote!(
        use parse_display::Display;
        use thiserror::Error as ThisError;
        use reqwest::StatusCode;
        use serde::Deserialize;

        #[derive(Display, Debug, ThisError)]
        #[display(style = "snake_case")]
        pub enum Error {
            #[display("reqwest error: {0}")]
            Reqwest(#[from] reqwest::Error),
            #[display("Unexpected http error: {code} {body}")]
            UnexpectedHttp { code: u16, body: String },
            #[dispaly("Unable to parse json: {json} {err}")]
            JsonParse {
                err: serde_json::Error,
                input: String,
            },
        }

        pub type Result<T> = error_stack::Result<T, Error>;
    )
}
