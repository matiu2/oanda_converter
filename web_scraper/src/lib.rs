pub mod definitions;
pub mod endpoint_docs;

use definitions::get_definitions;
use model::{Content, Documentation, Endpoint, ErrorDefinition, Everything};
use url::Url;
pub mod error;
use endpoint_docs::endpoint_docs;
pub use error::{Error, IntoReport, Result};
use error_stack::ResultExt;
use scraper::{Html, Selector};

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err(Error::default()).into_report().attach_printable_lazy(|| (format!($($arg)*)))
    };
}

#[macro_export]
macro_rules! report {
    ($($arg:tt)*) => {
        Err(Error::default()).into_report().attach_printable_lazy(|| format!($($arg)*))
    };
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
    // Get all the endpoint documentation
    let documentation = if url.path().ends_with("-ep/") {
        // Extract the endpoint name from the url
        let Some(name) = url_to_name(&url, "-ep") else {
            bail!("Unable to extract endpoint name from url: {url:#?}")
        };
        Documentation::Endpoint(Endpoint {
            calls: endpoint_docs(&document, name.as_str())?,
            name,
        })
    } else if url.path().ends_with("-df/") {
        let Some(name) = url_to_name(&url, "-df") else {
            bail!("Unable to extract definition name from url: {url:#?}")
        };
        Documentation::Definitions {
            name,
            definitions: get_definitions(&document)?,
        }
    } else {
        bail!("Unknown url ending: {url}");
    };
    Ok(Content {
        urls,
        documentation,
    })
}

/// Extracts the endpoint name from the url if possible
fn url_to_name(url: &Url, suffix: &str) -> Option<String> {
    Some(
        url.path_segments()?
            .filter(|segment| !segment.is_empty())
            .last()?
            .strip_suffix(suffix)?
            .to_string(),
    )
}

/// Gather the endpoint and definition links from the left side of the page
fn endpoint_links(document: &Html, base_url: &Url) -> Result<Vec<Url>> {
    let selector = scraper::Selector::parse("#resources-api-menu a").map_err(Error::from)?;
    document
        .select(&selector)
        .map(|element| {
            element
                .value()
                .attr("href")
                // If we find an ancchor element without an href, something has changed on the site and we should know about it
                .ok_or_else(Error::default)
                .into_report()
                .attach_printable_lazy(|| format!("Found an anchor without an href: {element:#?}"))
                .and_then(|href| {
                    // If it's an abolute url, it'll parse
                    Url::parse(href).or_else(|_| {
                        // If it's not an absolute url, try and join it to our base url
                        base_url.join(href).into_report().attach_printable_lazy(|| {
                            format!("Unable to parse url: {href} with base_url: {base_url:#?}")
                        })
                    })
                })
        })
        .collect()
}

/// Gets all content on the oanda site from all URLs
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
pub async fn get_all_content() -> Result<Everything> {
    let instrument_url =
        reqwest::Url::parse("https://developer.oanda.com/rest-live-v20/instrument-ep/").unwrap();

    let content = get_content(instrument_url.clone())
        .await
        .attach_printable_lazy(|| format!("At url: {instrument_url}"))?;

    let mut tasks = Vec::new();

    for url in content
        .urls
        .into_iter()
        .filter(|url| url != &instrument_url)
        // Forex labs just says "coming soon" at the time of writing -  Sat 27 May 2023 19:02:27 AEST
        .filter(|url| url.path() != "/rest-live-v20/forexlabs-ep/")
    {
        tasks.push(tokio::spawn(async move {
            get_content(url.clone())
                .await
                .attach_printable_lazy(|| format!("At url: {url}"))
        }));
    }

    let mut content = Vec::new();

    for result in futures::future::join_all(tasks).await {
        match result {
            Ok(Ok(contents)) => content.push(contents),
            Ok(Err(err)) => bail!("While getting content: {err:#?}"),
            Err(err) => bail!("Error while waiting for get_content: {err:#?}"),
        }
    }

    // Scrape the errors
    let errors = scrape_error_page().await?;

    Ok(Everything { content, errors })
}

async fn scrape_error_page() -> Result<Vec<ErrorDefinition>> {
    let html = reqwest::get("https://developer.oanda.com/rest-live-v20/troubleshooting-errors/")
        .await
        .into_report()
        .change_context(Error::default())?
        .text()
        .await
        .into_report()
        .change_context(Error::default())?;
    parse_error_page(&html)
}

fn parse_error_page(html: &str) -> Result<Vec<ErrorDefinition>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("#single-column *").map_err(Error::from)?;
    // First skip until the second <h4> tag
    let nodes: Vec<_> = document
        .select(&selector)
        .skip_while(|e| e.value().name() != "h4")
        // Skip over the actual <h4>table of contents</h4>
        .skip(1)
        .skip_while(|e| e.value().name() != "h4")
        .inspect(|e| println!("e: {} {}", e.value().name(), e.text().collect::<String>()))
        .collect();
    // Grab a section of text
    let text: String = nodes
        .iter()
        .take_while(|e| e.value().name() != "h4")
        .map(|e| e.text().collect::<String>())
        .collect();
    println!("text: {text}");
    // let mut sections = Vec::new();
    for (i, element) in nodes.iter().enumerate() {
        println!("Element {i}: {:#?}", element.value());
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::Documentation;

    use super::get_content;
    use error_stack::ResultExt;
    use model::{
        definition_docs::Schema,
        endpoint_docs::{Endpoints, HttpMethod},
        Endpoint,
    };
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
        // We're just reading the endpoint docs here:
        let Documentation::Endpoint(Endpoint { name, calls }) = &content.documentation else {
            panic!("Expected endpoint docs")
        };
        assert_eq!("instrument", name);
        let first_api_call_docs = &calls[0];
        assert_eq!(first_api_call_docs.http_method, HttpMethod::Get);
        assert_eq!(
            first_api_call_docs.doc_string.as_str(),
            "Fetch candlestick data for an instrument."
        );
        assert!(calls
            .iter()
            .all(|docs| docs.endpoint == Endpoints::Instrument));
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
        // Check the endpoint doc responses
        let candles = calls.first().unwrap();

        let ok_response = candles.responses.first().unwrap();
        assert_eq!(200, ok_response.code);
        assert_eq!(
            "Pricing information has been successfully provided.",
            &ok_response.description
        );
        let header = ok_response.headers.first().unwrap();
        assert_eq!("RequestID", &header.name);
        assert_eq!(
            "The unique identifier generated for the request",
            &header.description
        );
        let field = if let Schema::Struct(s) = &ok_response.schema {
            s.fields
                .iter()
                .find(|field| field.name == "candles")
                .unwrap()
        } else {
            panic!("Expected struct be got stream: {:#?}", ok_response.schema)
        };
        assert!(field.is_array);
        assert_eq!(
            "The list of candlesticks that satisfy the request.",
            &field.doc_string
        );
        assert_eq!("Candlestick", field.type_name);
        // Make sure it got the extra responses
        assert_eq!(&vec![400, 401, 404, 405,], &candles.other_responses);
    }

    #[tokio::test]
    async fn test_get_all_content() -> super::Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .try_init()
            .ok();
        let all_content = super::get_all_content().await?;
        println!("{all_content:#?}");
        Ok(())
    }

    #[test]
    fn test_parse_error_page() {
        let input = include_str!("../test_content/errors.html");
        super::parse_error_page(input).unwrap();
    }
}
