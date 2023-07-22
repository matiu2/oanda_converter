//! Generates error.rs for oanda_v2
use quote::{__private::TokenStream, quote};

pub fn gen_error() -> TokenStream {
    quote!(

        use error_stack::{IntoReport as ESIntoReport, ResultExt};
        use parse_display::Display;
        use reqwest::StatusCode;
        use serde::Deserialize;

        _blank_!();
        #[derive(Display, Debug, Default)]
        #[display(style = "snake_case")]
        pub enum Error {
            /// Most errors come from this category
            #[default]
            General,
            #[display("Message: {}")]
            Message(String),
            #[display("API Error: {status} {error}")]
            Api { status: StatusCode, error: ApiError },
        }

        _blank_!();
        #[derive(Display, Deserialize, Debug)]
        #[display(
            r#"error_codes: {error_codes:#?}
        error_uri: {error_uri}
        error_description: {error_description}
        trace_id: {trace_id}
        timestamp: {timestamp}
        error: {error}
        correlation_id: {correlation_id}"#
        )]

        _blank_!();
        pub struct ApiError {
            pub error_codes: Vec<usize>,
            pub error_uri: String,
            pub error_description: String,
            pub trace_id: String,
            pub timestamp: String,
            pub error: String,
            pub correlation_id: String,
        }

        _blank_!();
        impl std::error::Error for Error {}
        pub type Result<T> = error_stack::Result<T, Error>;

        _blank_!();
        pub trait IntoReport<T> {
            fn into_report(self) -> Result<T>;
        }

        _blank_!();
        impl<T, E> IntoReport<T> for std::result::Result<T, E>
        where
            std::result::Result<T, E>: ESIntoReport<Ok = T, Err = E>,
        {
            #[track_caller]
            fn into_report(self) -> Result<T> {
                ESIntoReport::into_report(self).change_context(Error::General)
            }
        }
    )
}
