//! Parses all the possible responses for an API call

use std::collections::HashMap;

use crate::{bail, definitions::read_struct, Error, Result};
use error_stack::{IntoReport, ResultExt};
use model::{
    defintion_docs::{Definition, Struct},
    endpoint_docs::{Response, ResponseHeader},
};
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
fn parse_response_title(a: ElementRef) -> Result<(u16, String)> {
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

/// Takes a .panel_group div and extracts the response data ot of it
fn parse_response_docs(panel: ElementRef) -> Result<Response> {
    let title_selector = Selector::parse(".panel-heading .panel-title a").map_err(Error::from)?;
    // Get the title
    let Some(a) = panel
        .select(&title_selector)
        .next()else { bail!("Couldn't find response title in {}", panel.html())};
    let (code, description) = parse_response_title(a)?;
    // Get the response http headers
    let headers_selector = Selector::parse(".panel ul li").map_err(Error::from)?;
    let headers = panel
        .select(&headers_selector)
        .flat_map(|li| li.text().next());
    todo!()
}

fn parse_response_http_header(li: ElementRef) -> Result<ResponseHeader> {
    let Some(text) = li.text().next() else {bail!("No text inside of li: {}", li.html())};
    let parts: Vec<&str> = text.splitn(2, "-").collect();
    match parts.as_slice() {
        [name, description] => Ok(ResponseHeader {
            name: name.trim().to_string(),
            description: description.trim().to_string(),
        }),
        _ => bail!(
            "Unable to extract name and description from response header html: {}",
            li.html()
        ),
    }
}

/// Given an div.endpoint_body documentation html block, extracts all of the responses
fn parse_response_docs_group(body: ElementRef) -> Result<Vec<Response>> {
    let response_selector = Selector::parse("div.panel_group > div.panel").map_err(Error::from)?;
    todo!("Iterate through the panels calling parse_response_docs");
    // Get the bodies
    let body_selector = Selector::parse("pre.json_schema").map_err(Error::from)?;
    let bodies = body
        .select(&body_selector)
        .map(read_struct)
        .collect::<Result<Vec<Struct>>>()?;
    // Now get the actual definiton table
    todo!()
}

#[cfg(test)]
mod unit_tests {
    use model::endpoint_docs::ResponseHeader;
    use pretty_assertions::assert_eq;
    use scraper::Html;

    #[test]
    fn test_parse_response_title() {
        let html = r##"
<a class="" role="button" data-toggle="collapse"
   data-parent="#accordion" href="#collapse_2_200"
   aria-expanded="false" aria-controls="collapse_2_200"
>
   <b>HTTP 200</b> – The list of Trades requested
</a>
        "##;
        let input = Html::parse_fragment(html);
        let (code, description) = super::parse_response_title(input.root_element()).unwrap();
        assert_eq!(200, code);
        assert_eq!("The list of Trades requested", description.as_str());
    }

    #[test]
    fn test_parse_response_http_header() {
        let html = r#"<li>Location - A link to the Order that was just created</li>"#;
        let input = Html::parse_fragment(html);
        let header = super::parse_response_http_header(input.root_element()).unwrap();
        assert_eq!(
            ResponseHeader {
                name: "Location".to_string(),
                description: "A link to the Order that was just created".to_string(),
            },
            header
        );
    }
}
