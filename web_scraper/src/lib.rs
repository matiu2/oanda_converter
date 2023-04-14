pub mod definitions;
pub mod endpoint_docs;

use model::endpoint_docs::RestCall;
use reqwest::Url;
pub mod error;
use endpoint_docs::endpoint_docs;
pub use error::Error;
use error_stack::{IntoReport, Result as ErrorStackResult, ResultExt};
use scraper::Html;

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        error_stack::bail!(Error::new(format!($($arg)*)))
    };
}

pub type Result<T> = ErrorStackResult<T, Error>;

pub struct Content {
    pub urls: Vec<Url>,
    pub endpoint_docs: Vec<RestCall>,
}

pub async fn get_content(url: Url) -> Result<Content> {
    let html = reqwest::get(url.clone())
        .await
        .into_report()
        .change_context(Error::default())?
        .text()
        .await
        .into_report()
        .change_context(Error::default())?;
    let document = Html::parse_document(&html);
    let urls = endpoint_links(&document, &url)?;
    // Extract the endpoint name from the url
    // Get all the endpoint documentation
    let Some(endpoint_name) = endpoint_name(&url) else { bail!("Unable to extract endpoint name from url: {url:#?}")};
    let endpoint_docs = endpoint_docs(&document, endpoint_name)?;
    println!("{endpoint_docs:#?}");
    Ok(Content {
        urls,
        endpoint_docs,
    })
}

/// Extracts the endpoint name from the url if possible
fn endpoint_name(url: &Url) -> Option<String> {
    Some(
        url.path_segments()?
            .filter(|segment| !segment.is_empty())
            .last()?
            .strip_suffix("-ep")?
            .to_string(),
    )
}

/// Gather the endpoint and definition links from the left side of the page
fn endpoint_links(document: &Html, base_url: &Url) -> Result<Vec<Url>> {
    let selector = scraper::Selector::parse("#resources-api-menu a").map_err(Error::from)?;
    document
        .select(&selector)
        .flat_map(|element| element.value().attr("href"))
        .map(|href| {
            // If it's an abolute url, it'll parse
            Url::parse(href).or_else(|_| {
                // If it's not an absolute url, try and join it to our base url
                base_url
                    .join(href)
                    .into_report()
                    .change_context(Error::new(format!(
                        "Unable to parse url: {href} with base_url: {base_url:#?}"
                    )))
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::get_content;
    use error_stack::ResultExt;
    use model::endpoint_docs::{Endpoint, HttpMethod};
    use reqwest::Url;

    #[tokio::test]
    async fn instrument_page() {
        let url = reqwest::Url::parse("https://developer.oanda.com/rest-live-v20/instrument-ep/")
            .unwrap();
        let content = get_content(url.clone())
            .await
            .attach_printable_lazy(|| format!("At url: {url}"))
            .unwrap();
        // Make sure it got the trade-ep URL
        assert!(content
            .urls
            .contains(&Url::parse("https://developer.oanda.com/rest-live-v20/trade-ep/").unwrap()));
        // Make sure it got the instrument-df URL
        assert!(content.urls.contains(
            &Url::parse("https://developer.oanda.com/rest-live-v20/instrument-df/").unwrap()
        ));
        let first_api_call_docs = &content.endpoint_docs[0];
        assert_eq!(first_api_call_docs.http_method, HttpMethod::Get);
        assert_eq!(
            first_api_call_docs.doc_string.as_str(),
            "Fetch candlestick data for an instrument."
        );
        assert!(content
            .endpoint_docs
            .iter()
            .all(|docs| docs.endpoint == Endpoint::Instrument));
        assert_eq!(
            first_api_call_docs.path.as_str(),
            "/v3/instruments/{instrument}/candles"
        );
        let third_parameter = &first_api_call_docs.parameters[3];
        assert_eq!(third_parameter.type_name.as_str(), "PricingComponent");
        assert_eq!(
            third_parameter.description.as_str(),
            "The Price component(s) to get candlestick data for. [default=M]"
        );
    }
}
