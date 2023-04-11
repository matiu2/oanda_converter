use reqwest::Url;
pub mod error;
pub use error::Error;
use error_stack::{bail, IntoReport, Result as ErrorStackResult, ResultExt};
use scraper::{Html, Selector};
use types::EndpointDoc;

pub type Result<T> = ErrorStackResult<T, Error>;

// TODO: Move all of these out into their own library
pub mod types {

    pub struct EndpointDoc {
        pub http_method: HttpMethod,
    }

    pub enum HttpMethod {
        Get,
        Post,
    }
}

async fn get_content(url: Url) -> Result<()> {
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
    let endpoint_docs = endpoint_docs(&document)?;
    Ok(())
}

/// Get the documentation for the endpoint docs
fn endpoint_docs(document: &Html) -> Result<Vec<EndpointDoc>> {
    let top_selector = Selector::parse("#single-column > div").map_err(Error::from)?;
    let method_selector = Selector::parse(".method").map_err(Error::from)?;
    let Some(top_fragment) = document.select(&top_selector).next()  else { return Ok(vec!()) };
    let Some(method) = top_fragment.select(&method_selector).next() else { bail!(Error::new("Unable to find http method"))};
    let Some(method) = method.text().next() else { bail!(Error::new("Couldn't extract the method text"))};
    println!("Got HTTP method: {method}");
    Ok(vec![])
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

// /// Read the get and post requests
// async fn api_reader(&self, _response: Response, a: ElementRef) -> Result<()> {
//     println!("Got api section");
//     for element in a.select(".endpoint_header .path, .endpoint_body th") {
//         println!("{:?}: {:?}", element.tag(), element.text());
//     }
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::get_content;

    #[tokio::test]
    async fn it_works() {
        let url = reqwest::Url::parse("https://developer.oanda.com/rest-live-v20/instrument-ep/")
            .unwrap();
        get_content(url).await.unwrap();
    }
}
