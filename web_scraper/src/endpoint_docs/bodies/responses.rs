//! Parses all the possible responses for an API call

use std::collections::HashMap;

use crate::{bail, definitions::read_struct, Error, Result};
use error_stack::{IntoReport, ResultExt};
use model::defintion_docs::{Definition, Struct};
use scraper::{ElementRef, Selector};

/// Takes an HTML like:
///
/// ```html
/// <a class="" role="button" data-toggle="collapse"
///    data-parent="#accordion" href="#collapse_2_200"
///    aria-expanded="false" aria-controls="collapse_2_200"
/// >
///    <b>HTTP 200</b> – The list of Trades requested
/// </a>
/// ```
/// And extracts the (200, "The list of Trades requested")
fn parse_response_header(a: ElementRef) -> Result<(u16, String)> {
    let code_selector = Selector::parse("b").map_err(Error::from)?;
    let Some(code) = a.select(&code_selector).next() else { bail!("No <b> holding the reponse code while parsing response docs: {}", a.html())};
    // The code should be in the format "HTTP 200" - We just want the 200
    let get_code = || code.text().next()?.split_whitespace().nth(1);
    let Some(code) = get_code() else { bail!("Unable to get the code out of: {}", code.html())};
    let code: u16 = code
        .parse()
        .into_report()
        .change_context(Error::default())?;
    // Now get the description for it
    let description: String = a
        .text()
        .map(str::trim)
        .filter(|text| !text.trim().is_empty() && !text.starts_with("HTTP"))
        .flat_map(|text| text.strip_prefix("– "))
        .inspect(|text| println!("Text: {text}"))
        .collect();
    Ok((code, description))
}

fn parse_response_docs(div: ElementRef) -> Result<HashMap<u16, Definition>> {
    let headers_selector = Selector::parse("[role=\"tab\"] a").map_err(Error::from)?;
    // Get the headers
    let headers = div
        .select(&headers_selector)
        .map(parse_response_header)
        .collect::<Result<Vec<(u16, String)>>>()?;
    // Get the bodies
    let body_selector = Selector::parse("pre.json_schema").map_err(Error::from)?;
    let bodies = div
        .select(&body_selector)
        .map(read_struct)
        .collect::<Result<Vec<Struct>>>()?;
    // Now get the actual definiton table
    todo!()
}

#[cfg(test)]
mod unit_tests {
    use scraper::Html;

    #[test]
    fn test_parse_response_header() {
        let html = r##"
<a class="" role="button" data-toggle="collapse"
   data-parent="#accordion" href="#collapse_2_200"
   aria-expanded="false" aria-controls="collapse_2_200"
>
   <b>HTTP 200</b> – The list of Trades requested
</a>
        "##;
        let input = Html::parse_fragment(html);
        let (code, description) = super::parse_response_header(input.root_element()).unwrap();
        assert_eq!(200, code);
        assert_eq!("The list of Trades requested", description.as_str());
    }
}
