//! Parses all the possible responses for an API call

use std::collections::HashSet;

use crate::{bail, definitions::read_struct, Error, Result};
use error_stack::{IntoReport, ResultExt};
use model::{
    definition_docs::{Schema, Stream},
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
        .collect();
    Ok((code, description))
}

/// Takes a .panel div for a single response and extracts the data out of it
fn parse_single_response_doc(panel: ElementRef) -> Result<Response> {
    let title_selector = Selector::parse(".panel-heading .panel-title a").map_err(Error::from)?;
    // Get the title
    let Some(a) = panel
        .select(&title_selector)
        .next() else { bail!("Couldn't find response title in {}", panel.html())};
    let (code, description) = parse_response_title(a)?;
    // Get the response http headers
    let headers_selector = Selector::parse(".panel ul li").map_err(Error::from)?;
    let headers = panel
        .select(&headers_selector)
        .map(parse_response_http_header)
        .collect::<Result<Vec<ResponseHeader>>>()
        .attach_printable_lazy(|| format!("In {code} {description}"))?;
    // Get the body
    let body_selector = Selector::parse(".panel-body pre.json_schema").map_err(Error::from)?;
    // TODO: in https://developer.oanda.com/rest-live-v20/transaction-ep/ for /v3/accounts/{accountID}/transactions/stream
    // It has a stream. We need to parse this differently
    let schema = if let Some(body) = panel.select(&body_selector).next() {
        Schema::Struct(read_struct(body)?)
    } else {
        // If we can't find a straight schema, it must be a stream
        let stream_selector =
            Selector::parse(".panel-body > div > ul > li > a").map_err(Error::from)?;
        let objects = panel
            .select(&stream_selector)
            .map(|a| {
                a.text()
                    .next()
                    .map(str::to_string)
                    .ok_or(Error::default())
                    .into_report()
                    .attach_printable_lazy(|| format!("Found empty object_name in {}", a.html()))
            })
            .collect::<Result<HashSet<String>>>()?;
        Schema::Stream(Stream { objects })
    };
    Ok(Response {
        code,
        description,
        headers,
        schema,
    })
}

fn parse_response_http_header(li: ElementRef) -> Result<ResponseHeader> {
    let Some(text) = li.text().next() else {bail!("No text inside of li: {}", li.html())};
    let parts: Vec<&str> = text.splitn(2, '-').collect();
    match parts.as_slice() {
        [name, description] => Ok(ResponseHeader {
            name: name.trim().to_string(),
            description: description.trim().to_string(),
        }),
        [name] => Ok(ResponseHeader {
            name: name.trim().to_string(),
            description: String::new(),
        }),
        _ => bail!(
            "Unable to extract name and description from response header html: {}",
            li.html()
        ),
    }
}

/// Given an div.endpoint_body documentation html block, extracts all of the responses
pub(crate) fn parse_responses_docs_group(body: &ElementRef) -> Result<Vec<Response>> {
    let response_selector =
        Selector::parse("div.panel-group:nth-child(4) > div.panel").map_err(Error::from)?;
    body.select(&response_selector)
        .map(parse_single_response_doc)
        .collect()
}

/// Given a .endpoint_body extracts the extra resopnse http codes
pub(crate) fn parse_extra_resonses(body: &ElementRef) -> Result<Vec<u16>> {
    // Get the other http responses that it can return
    let extra_responses_selector =
        Selector::parse(".endpoint_body > p > a").map_err(Error::from)?;
    body.select(&extra_responses_selector)
        .flat_map(|a| a.text())
        .map(|code| code.parse().into_report().change_context(Error::default()))
        .collect()
}

#[cfg(test)]
mod unit_tests {
    use crate::{IntoReport, ResultExt};
    use model::{definition_docs::Schema, endpoint_docs::ResponseHeader};
    use pretty_assertions::assert_eq;
    use scraper::Html;

    use crate::{bail, Error, Result};

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

    #[test]
    fn test_parse_response_http_header_no_description() {
        let html = r#"<li><a href="../transaction-df/#Transaction">Transaction</a>
</li>"#;
        let input = Html::parse_fragment(html);
        let header = super::parse_response_http_header(input.root_element()).unwrap();
        assert_eq!(
            ResponseHeader {
                name: "Transaction".to_string(),
                description: String::new(),
            },
            header
        );
    }

    #[test]
    fn test_parse_single_response_doc() -> Result<()> {
        let input = r##"
            <div class="panel panel-default">
<div class="panel-heading" role="tab" id="heading_2_201">
<span class="panel-title">
<a class="" role="button" data-toggle="collapse" data-parent="#accordion" href="#collapse_2_201" aria-expanded="false" aria-controls="collapse_2_201">
<b>HTTP 201</b> – The Order was created as specified
</a>
</span>
</div>
<div id="collapse_2_201" class="panel-collapse collapse in" role="tabpanel" aria-labelledby="heading_2_201" style="">
<div class="panel-body">
<b>Response Headers</b>
<p>
</p><ul>
<li>Location - A link to the Order that was just created</li>
<li>RequestID - The unique identifier generated for the request</li>
</ul>
<b>Response Body Schema (application/json)</b>
<p>
</p><pre class="json_schema">{
    # 
    # The Transaction that created the Order specified by the request.
    # 
    orderCreateTransaction : (<a href="../transaction-df/#Transaction">Transaction</a>),

    # 
    # The Transaction that filled the newly created Order. Only provided when
    # the Order was immediately filled.
    # 
    orderFillTransaction : (<a href="../transaction-df/#OrderFillTransaction">OrderFillTransaction</a>),

    # 
    # The Transaction that cancelled the newly created Order. Only provided
    # when the Order was immediately cancelled.
    # 
    orderCancelTransaction : (<a href="../transaction-df/#OrderCancelTransaction">OrderCancelTransaction</a>),

    # 
    # The Transaction that reissues the Order. Only provided when the Order is
    # configured to be reissued for its remaining units after a partial fill
    # and the reissue was successful.
    # 
    orderReissueTransaction : (<a href="../transaction-df/#Transaction">Transaction</a>),

    # 
    # The Transaction that rejects the reissue of the Order. Only provided when
    # the Order is configured to be reissued for its remaining units after a
    # partial fill and the reissue was rejected.
    # 
    orderReissueRejectTransaction : (<a href="../transaction-df/#Transaction">Transaction</a>),

    # 
    # The IDs of all Transactions that were created while satisfying the
    # request.
    # 
    relatedTransactionIDs : (Array[<a href="../transaction-df/#TransactionID">TransactionID</a>]),

    # 
    # The ID of the most recent Transaction created for the Account
    # 
    lastTransactionID : (<a href="../transaction-df/#TransactionID">TransactionID</a>)
}
</pre>
</div> </div></div>
        "##;
        let html = Html::parse_fragment(input);
        let panel = html.root_element();
        let response = super::parse_single_response_doc(panel).unwrap();
        assert_eq!(201, response.code);
        assert_eq!("The Order was created as specified", &response.description);
        assert!(response
            .headers
            .iter()
            .any(|header| &header.name == "RequestID"));
        let field = if let Schema::Struct(s) = &response.schema {
            s.fields
                .iter()
                .find(|field| field.name == "lastTransactionID")
                .unwrap()
        } else {
            bail!(
                "Expected a struct, but got a stream: {:#?}",
                response.schema
            )
        };
        assert_eq!("TransactionID", field.type_name);
        assert_eq!(
            "The ID of the most recent Transaction created for the Account",
            field.doc_string
        );
        Ok(())
    }

    #[test]
    fn test_read_struct() -> Result<()> {
        let input = r#"{
     #
     # The Account’s identifier
     # <b>Amazing</b>: Do use it dude.
     #
     id : (<a href="../account-df/#AccountID">AccountID</a>),
}"#;
        let fragment = Html::parse_fragment(input);
        let got = super::read_struct(fragment.root_element())?;
        assert_eq!(1, got.fields.len(), "There should only be one field");
        let Some(id) = got.fields.iter().find(|field| field.name == "id") else { bail!("No ID field in {got:#?}")};
        assert_eq!("id", id.name.as_str());
        assert_eq!("AccountID", id.type_name.as_str());
        assert_eq!(
            "The Account’s identifier Amazing: Do use it dude.",
            id.doc_string.as_str()
        );
        assert!(!id.is_array);
        assert!(id.default.is_none());
        assert!(!id.required);
        Ok(())
    }
}
