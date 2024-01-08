use crate::{host::Host, Error};
use error_stack::{Report, ResultExt};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use std::borrow::ToOwned;

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

    /// Given a URL path, creates a get request builder with the
    /// correct host and authentication token
    pub fn get(&self, url: &str) -> RequestBuilder {
        use reqwest::header::{ACCEPT, AUTHORIZATION};
        self.rest_client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", &self.token))
            .header(ACCEPT, "application/json")
    }

    /// Given a URL path, creates a put request builder with the
    /// correct host and authentication token
    pub fn put(&self, url: &str) -> RequestBuilder {
        use reqwest::header::{ACCEPT, AUTHORIZATION};
        self.rest_client
            .put(url)
            .header(AUTHORIZATION, format!("Bearer {}", &self.token))
            .header(ACCEPT, "application/json")
    }

    /// Given a URL path, creates a patch request builder with the
    /// correct host and authentication token
    pub fn patch(&self, url: &str) -> RequestBuilder {
        use reqwest::header::{ACCEPT, AUTHORIZATION};
        self.rest_client
            .patch(url)
            .header(AUTHORIZATION, format!("Bearer {}", &self.token))
            .header(ACCEPT, "application/json")
    }

    /// Given a URL path, creates a post request builder with the
    /// correct host and authentication token
    pub fn post(&self, url: &str) -> RequestBuilder {
        use reqwest::header::{ACCEPT, AUTHORIZATION};
        self.rest_client
            .post(url)
            .header(AUTHORIZATION, format!("Bearer {}", &self.token))
            .header(ACCEPT, "application/json")
    }

    /// Given a URL path, creates a delete request builder with the
    /// correct host and authentication token
    pub fn delete(&self, url: &str) -> RequestBuilder {
        use reqwest::header::{ACCEPT, AUTHORIZATION};
        self.rest_client
            .delete(url)
            .header(AUTHORIZATION, format!("Bearer {}", &self.token))
            .header(ACCEPT, "application/json")
    }

    /// Makes an authenticated get request to a path in the rest api
    pub async fn send<T: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> error_stack::Result<T, Error> {
        let request = request
            .build()
            .map_err(Report::from)
            .change_context_lazy(|| Error::new("Building request"))?;
        let url = request.url().to_owned();
        let response = self
            .rest_client
            .execute(request)
            .await
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("URL: {url}")))?;
        let status = response.status();
        let body = response.text().await;
        if status.is_client_error() {
            let result = Err(Error::new("HTTP Status code indicates client error"));
            match body {
                Ok(body) => result.attach_printable_lazy(|| format!("Body: {body}")),
                Err(body) => result.attach_printable_lazy(|| format!("Error getting body: {body}")),
            }
        } else if status.is_server_error() {
            let result = Err(Error::new("HTTP Status code indicates server error"));
            match body {
                Ok(body) => result.attach_printable_lazy(|| format!("Body: {body}")),
                Err(body) => result.attach_printable_lazy(|| format!("Error getting body: {body}")),
            }
        } else {
            match body {
                Ok(body) => serde_json::from_str(&body)
                    .map_err(Report::from)
                    .change_context_lazy(|| Error::new("Parsing json"))
                    .attach_printable_lazy(|| format!("Body: {body}")),
                Err(err) => Err(Report::from(err))
                    .change_context_lazy(|| Error::new("Retrieving HTTP body")),
            }
        }
        .attach_printable_lazy(|| format!("HTTP status code: {status:#?}"))
        .attach_printable_lazy(|| format!("URL: {url}"))
    }
}
