use crate::{Error, Result};
use change_case::pascal_case;
use error_stack::ResultExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use utils::pretty_doc_string;

pub fn gen_client(mods: &[&str]) -> Result<TokenStream> {
    let mods: Vec<TokenStream> = mods
        .iter()
        .map(|name| {
            let m = Ident::new(name, proc_macro2::Span::call_site());
            let s = Ident::new(&pascal_case(name), proc_macro2::Span::call_site());
            quote!(use crate::endpoints::#m::#s;)
        })
        .collect();

    let builders = ["get", "put", "patch", "post", "delete"].into_iter().map(|http_method| {
        let ident = Ident::new(http_method, Span::call_site());
        let doc_string = pretty_doc_string(&format!("Given a URL path, creates a {http_method} request builder with the correct host and authentication token")).
        change_context_lazy(|| Error::new(format!("Creating doc string for {http_method} builder in gen_client")))?;
        Ok(quote! {
            #(#doc_string)*
            pub fn #ident(&self, url: &str) -> RequestBuilder {
                use reqwest::header::{ACCEPT, AUTHORIZATION};
                self.rest_client
                    .#ident(url)
                    .header(AUTHORIZATION, format!("Bearer {}", &self.token))
                    .header(ACCEPT, "application/json")
            }
        })
    }).collect::<Result<Vec<TokenStream>>>()?;

    Ok(quote!(
        use std::borrow::ToOwned;
        use error_stack::{report, ResultExt, Report};
        use reqwest::RequestBuilder;
        use serde::de::DeserializeOwned;
        use crate::{Error, host::Host, Result};
        #(#mods)*

        #[derive(Debug, Clone)]
        pub struct Client {
            token: String,
            pub host: Host,
            rest_client: reqwest::Client,
        }

        impl Client {
            /// Creates a new [`Client`].
            ///
            /// `token` is your API Token
            /// `host` is the host to use
            pub fn new(token: String, host: Host) -> Client {
                let rest_client = reqwest::Client::builder()
                    .deflate(true)
                    .gzip(true)
                    .brotli(true)
                    .build()
                    .map_err(Report::from)
                    .unwrap();
                Client {
                    token,
                    host,
                    rest_client,
                }
            }

            /// Given a URL path, inserts the part before it
            pub fn url(&self, path: &str) -> String {
                self.host.rest_url(path)
            }

            #(#builders)*

            /// Makes an authenticated get request to a path in the rest api
            pub async fn send<T: DeserializeOwned>(
                &self,
                request: RequestBuilder,
            ) -> error_stack::Result<T, Error> {
                let request = request.build().map_err(Error::from).map_err(Report::from)?;
                let url = request.url().to_owned();

                let response = self
                    .rest_client
                    .execute(request)
                    .await
                    .map_err(Error::from)
                    .map_err(Report::from)
                    .attach_printable_lazy(|| format!("URL: {url}"))?;

                let status = response.status();

                // Try and get the body, regardless if there was a bad status code
                let body: Result<String> = response
                    .text()
                    .await
                    .map_err(Error::from)
                    .map_err(Report::from)
                    .attach_printable_lazy(|| format!("URL: {url}"))
                    .attach_printable_lazy(|| format!("HTTP status code: {status}"));

                match response.error_for_status_ref() {
                    Ok(response) => {
                        let body = body?;
                        // Try to parse the json separetely from reqwest
                        serde_json::from_str(&body)
                            .map_err(|err| Error::JsonParse {
                                err,
                                input: body.to_owned(),
                            })
                            .map_err(Report::from)
                            .attach_printable_lazy(|| format!("HTTP status code: {status}"))
                            .attach_printable_lazy(|| format!("url: {url}"))
                    }
                    Err(err) => {
                        let err = Err(Error::from(err))
                            .map_err(Report::from)
                            .attach_printable_lazy(|| format!("HTTP status code: {status}"))
                            .attach_printable_lazy(|| format!("url: {url}"));
                        Err(match body {
                            Ok(body) => {
                                err.attach_printable(format!("Body: {body}"));
                                err
                            }
                            Err(body_err) => {
                                err.change_context(body_err);
                                err
                            }
                        })
                    }
                }
            }

            // /// Rest API for anything account related
            // pub fn accounts(&self) -> Accounts {
            //     Accounts { client: self }
            // }

            // /// Rest API for anything instrument related
            // pub fn instrument(&self, instrument: impl ToString) -> Instrument {
            //     Instrument {
            //         client: self,
            //         instrument: instrument.to_string(),
            //     }
            // }

            // /// Rest API for anything trade related including closing an existing Trade
            // pub fn trade(&self, account_id: impl ToString) -> Trade {
            //     Trade::new(self, account_id.to_string())
            // }

            // // Rest API for anything order related including openning a new position
            // pub fn order(&self, account_id: impl ToString) -> Order {
            //     Order::new(self, account_id.to_string())
            // }
        }


        #[cfg(test)]
        mod test_utils {
            use crate::{Client, Error};
            use error_stack::{IntoReport, Result, ResultExt};
            use lazy_static::lazy_static;
            use std::sync::Mutex;


            lazy_static! {
                static ref ACCOUNT_ID: Mutex<Option<String>> = Mutex::new(None);
            }


            pub async fn get_account_id(client: &Client) -> Result<String, Error> {
                let mut account_id = ACCOUNT_ID.lock().unwrap();
                if let Some(account_id) = account_id.as_ref() {
                    Ok(account_id.clone())
                } else {
                    let accounts = client.accounts().list().await?;
                    let out = accounts
                        .into_iter()
                        .next()
                        .ok_or_else(|| Error::Other)
                        .map_err(Report::from)
                        .attach_printable_lazy(|| "No oanda accounts found")?
                        .id;
                    *account_id = Some(out.clone());
                    Ok(out)
                }
            }
        }
    ))
}
