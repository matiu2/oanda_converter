pub mod endpoint_docs;

use reqwest::Url;
pub mod error;
use endpoint_docs::endpoint_docs;
pub use error::Error;
use error_stack::{bail, IntoReport, Result as ErrorStackResult, ResultExt};
use scraper::Html;

pub type Result<T> = ErrorStackResult<T, Error>;

pub async fn get_content(url: Url) -> Result<()> {
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
    let Some(endpoint_name) = endpoint_name(&url) else { bail!(Error::new(format!("Unable to extract endpoint name from url: {url:#?}")))};
    let endpoint_docs = endpoint_docs(&document, endpoint_name)?;
    println!("{endpoint_docs:#?}");
    Ok(())
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
    let selector = scraper::Selector::parse("#resources-api-menu #accordion:nth-child(2) a")
        .map_err(Error::from)?;
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

    #[tokio::test]
    async fn it_works() {
        let url = reqwest::Url::parse("https://developer.oanda.com/rest-live-v20/instrument-ep/")
            .unwrap();
        get_content(url.clone())
            .await
            .attach_printable_lazy(|| format!("At url: {url}"))
            .unwrap();
    }
}
