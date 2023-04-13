pub mod endpoint_docs;

use model::RestCall;
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

/// Read and navidgae the links to endpoint on the left side of the page
fn endpoint_links(document: &Html, base_url: &Url) -> Result<Vec<Url>> {
    let selector = scraper::Selector::parse("#resources-api-menu a").map_err(Error::from)?;
    let mut out = Vec::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            // Create absolute URL
            let url = match Url::parse(&href) {
                Ok(url) => url,
                Err(_) => base_url
                    .join(&href)
                    .into_report()
                    .change_context(Error::default())?,
            };
            out.push(url);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::get_content;
    use error_stack::ResultExt;
    use model::{Endpoint, HttpMethod};
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
