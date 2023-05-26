//! Scrapes the endpoint docs. URLs that end in "-ep/".

use crate::{annotate, bail, Error, Result};
use error_stack::{IntoReport, ResultExt};
use model::endpoint_docs::RestCall;
use scraper::Html;

mod bodies;
mod headers;

/// Get the documentation for the endpoint docs
pub fn endpoint_docs(document: &Html, name: &str) -> Result<Vec<RestCall>> {
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
                endpoint: annotate!(name.parse(), "While parsing {name}")?,
                http_method: annotate!(
                    header.http_method.parse(),
                    "While parsing {}",
                    header.http_method
                )?,
                path: header.path,
                doc_string: header.doc_string,
                responses: body.responses,
                parameters: body.parameters,
                other_responses: body.other_responses,
            })
        })
        .collect()
}
