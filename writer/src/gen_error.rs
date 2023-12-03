//! Generates error.rs for oanda_v2
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_error() -> TokenStream {
    quote!(

        use parse_display::Display;
        use reqwest::StatusCode;
        use serde::Deserialize;

        #[derive(Display, Debug, Default)]
        #[display(style = "snake_case")]
        pub enum Error {
            #[display("reqwest error: {0}")]
            Reqwest(#[from] reqwest::Error),
            #[display("Unexpected http error: {code} {body}")]
            UnexpectedHttp{code: u16, body: String},
        }

        impl std::error::Error for Error {}
        pub type Result<T> = error_stack::Result<T, Error>;

    )
}
