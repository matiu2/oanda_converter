use crate::{bail, Error, Result};
use error_stack::{IntoReport, ResultExt};
use model::endpoint_docs::{RestCall, RestCallParameter};
use scraper::{ElementRef, Html, Selector};

#[derive(Debug)]
struct Header {
    http_method: String,
    path: String,
    doc_string: String,
}

/// Get the documentation for the endpoint docs
pub fn endpoint_docs(document: &Html, name: String) -> Result<Vec<RestCall>> {
    // Get the HTTP method and path from the headers
    let headers = get_rest_call_headers(document)?;
    println!("{headers:#?}");
    // Get all the parameter tables
    let parameter_sets = get_all_rest_call_parameters(document)?;
    // Now zip them together
    if headers.len() != parameter_sets.len() {
        bail!(
            "The number of table headers and table bodies differs: {} headers and {} bodies",
            headers.len(),
            parameter_sets.len()
        )
    }
    headers
        .into_iter()
        .zip(parameter_sets)
        .map(|(header, parameters)| {
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
                parameters,
            })
        })
        .collect()
}

/// Get's the header part of each API call description
fn get_rest_call_headers(document: &Html) -> Result<Vec<Header>> {
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

fn get_all_rest_call_parameters(document: &Html) -> Result<Vec<Vec<RestCallParameter>>> {
    let body_selector =
        Selector::parse("#single-column > div.endpoint_body").map_err(Error::from)?;
    let th_selector = Selector::parse("th").map_err(Error::from)?;
    let tbody_selector = Selector::parse("tbody").map_err(Error::from)?;
    document
        .select(&body_selector)
        .map(|fragment| {
            // Just make sure they haven't changed up the table rows on us
            let table_rows: Vec<&str> = fragment
                .select(&th_selector)
                .flat_map(|tr| tr.text().next())
                .collect();
            const EXPECTED: [&str; 4] = ["Name", "Located In", "Type", "Description"];
            if table_rows.as_slice() != EXPECTED {
                bail!(
                    "The table rows have changed. \nExpected: {EXPECTED:?}\nGot:    {table_rows:?}"
                )
            }
            // Now read each row
            let Some(tbody) = fragment.select(&tbody_selector).next() else { bail!("Couldn't find table body for api call docs: using {tbody_selector:?} in {}", fragment.html())};
            get_rest_call_parameters(tbody)
        })
        .collect()
}

/// Parses each row of the table despcribing the parameters to a single rest API call
fn get_rest_call_parameters(tbody: ElementRef) -> Result<Vec<RestCallParameter>> {
    let td_selector = Selector::parse("td").map_err(Error::from)?;
    let tr_selector = Selector::parse("tr").map_err(Error::from)?;
    let get_tds = |tr: ElementRef| -> Vec<String> {
        tr.select(&td_selector)
            .flat_map(|td| Some(td.text().next()?.trim().to_string()))
            .collect()
    };
    tbody
        .select(&tr_selector)
        .map(|tr| {
            let tds = get_tds(tr);
            let [name, located_in, type_name, description] =
                <[String; 4]>::try_from(tds).map_err(|err| {
                    Error::new(format!(
                        "Unable to parse rest call parameter table row: {err:?}"
                    ))
                })?;
            Ok(RestCallParameter {
                name,
                located_in: located_in
                    .parse()
                    .into_report()
                    .change_context(Error::default())?,
                type_name,
                description,
            })
        })
        .collect()
}
