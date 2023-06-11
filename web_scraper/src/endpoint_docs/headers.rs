//! Parses the header part of each Rest API call documentation

use crate::{bail, Error, IntoReport, Result};
use error_stack::ResultExt;
use scraper::{Html, Selector};

#[derive(Debug)]
pub(crate) struct Header {
    pub http_method: String,
    pub path: String,
    pub doc_string: String,
}

/// Get's the header part of each API call description
pub(crate) fn get_rest_call_headers(document: &Html) -> Result<Vec<Header>> {
    let header_selector =
        Selector::parse("#single-column > div.endpoint_header").map_err(Error::from)?;
    let http_method_selector = Selector::parse(".method").map_err(Error::from)?;
    let path_selector = Selector::parse(".path").map_err(Error::from)?;
    let doc_string_selector = Selector::parse(".path > p").map_err(Error::from)?;
    document.select(&header_selector).map(|header_fragment| {
    let get_text = |selector| {
        Some(
            header_fragment
                .select(selector)
                .next()?
                .text()
                .next()?
                .to_string(),
        )
    };
    // Is it a header or a body
    // Get the http method fragment
    let Some(http_method) = get_text(&http_method_selector) else {
        bail!("Couldn't find the http method, with {http_method_selector:#?}: {}", header_fragment.html()[..1000].to_string()); 
    };
    // Read the path
    let Some(path) = get_text(&path_selector) else {
        bail!("Couldn't find the path with {path_selector:#?}: {}", header_fragment.html());
    };
    // Read the docstring
    let Some(doc_string) = get_text(&doc_string_selector) else {
        bail!("Couldn't find the doc_string with {doc_string_selector:#?}: {}", header_fragment.html());
    };
    Ok(Header{http_method, path, doc_string })
}).collect()
}
