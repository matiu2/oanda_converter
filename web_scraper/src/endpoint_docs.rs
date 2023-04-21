//! Scrapes the endpoint docs. URLs that end in "-ep/".

use crate::{bail, Error, Result};
use error_stack::{IntoReport, ResultExt};
use model::endpoint_docs::RestCall;
use scraper::Html;

mod bodies;
mod headers;

/// Get the documentation for the endpoint docs
pub fn endpoint_docs(document: &Html, name: String) -> Result<Vec<RestCall>> {
    // Get the HTTP method and path from the headers
    let headers = headers::get_rest_call_headers(document)?;
    // Get all the parameter tables
    let bodies = bodies::get_all_rest_call_bodies(document)?;
    // Make sure they're all the same length
    if headers.len() != bodies.len() {
        bail!(
            "The number of table headers, table bodies, and responses differs: {} headers and {} bodies",
            headers.len(),
            bodies.len()
        )
    }
    // Now zip them together
    headers
        .into_iter()
        .zip(bodies)
        .map(|(header, body)| {
            Ok(RestCall {
                endpoint: name
                    .parse()
                    .into_report()
                    .change_context(Error::default())?,
                http_method: header
                    .http_method
                    .parse()
                    .into_report()
                    .change_context(Error::default())?,
                path: header.path,
                doc_string: header.doc_string,
                responses: body.responses,
                parameters: body.parameters,
                other_responses: body.other_responses,
            })
        })
        .collect()
}
