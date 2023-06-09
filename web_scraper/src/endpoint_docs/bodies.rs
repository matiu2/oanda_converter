//! Parses the documentation for Rest call bodies, including:
//! * Header
//! * Parameters
//! * Responses
use crate::{bail, Error, Result};
use error_stack::{IntoReport, ResultExt};
use model::endpoint_docs::Response;
use model::endpoint_docs::RestCallParameter;
use scraper::{ElementRef, Html, Selector};

use responses::parse_responses_docs_group;

use self::responses::parse_extra_resonses;

mod responses;

/// Just the body part of the RestCall docs
pub(crate) struct RestCallBody {
    pub parameters: Vec<RestCallParameter>,
    pub responses: Vec<Response>,
    pub other_responses: Vec<u16>,
}

/// Given an html document representing a single endpoint,
/// finds each REST API call documentation block, and parses and returns it
pub(crate) fn get_all_rest_call_bodies(document: &Html) -> Result<Vec<RestCallBody>> {
    let response_selector =
        Selector::parse("#single-column > div.endpoint_body").map_err(Error::from)?;
    let bodies: Vec<ElementRef> = document.select(&response_selector).collect();
    // Get all the HTTP parameter definitions
    let all_parameters = bodies
        .iter()
        .map(get_rest_call_parameters)
        .collect::<Result<Vec<Vec<RestCallParameter>>>>()?;
    // Get all the responses for all the API calls in this endpoint
    let responses = bodies
        .iter()
        .map(parse_responses_docs_group)
        .collect::<Result<Vec<Vec<Response>>>>()?;
    let all_other_responses = bodies
        .iter()
        .map(parse_extra_resonses)
        .collect::<Result<Vec<Vec<u16>>>>()?;
    Ok(all_parameters
        .into_iter()
        .zip(responses)
        .zip(all_other_responses)
        .map(|((parameters, responses), other_responses)| RestCallBody {
            parameters,
            responses,
            other_responses,
        })
        .collect())
}

/// Given a .endpoint_body div, extracts the table headers and rows, which are the
/// Rest API call parameters
pub(crate) fn get_rest_call_parameters(body: &ElementRef) -> Result<Vec<RestCallParameter>> {
    let th_selector = Selector::parse("th").map_err(Error::from)?;
    let tbody_selector = Selector::parse("tbody").map_err(Error::from)?;
    // Just make sure they haven't changed up the table rows on us
    let table_rows: Vec<&str> = body
        .select(&th_selector)
        .flat_map(|tr| tr.text().next())
        .collect();
    const EXPECTED: [&str; 4] = ["Name", "Located In", "Type", "Description"];
    if table_rows.as_slice() != EXPECTED {
        bail!("The table rows have changed. \nExpected: {EXPECTED:?}\nGot:    {table_rows:?}")
    }
    // Now read each row
    let Some(tbody) = body.select(&tbody_selector).next() else
        { bail!("Couldn't find table body for api call docs: using {tbody_selector:?} in {}", body.html())};

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
            let [name, located_in, type_name, description] = <[String; 4]>::try_from(tds)
                .map_err(|err| {
                    Error::Message(format!(
                        "Unable to parse rest call parameter table row: {err:?}"
                    ))
                })
                .into_report()?;
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
