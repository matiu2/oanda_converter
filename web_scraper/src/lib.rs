use reqwest::Url;
pub mod error;
pub use error::Error;
use error_stack::{IntoReport, Result, ResultExt};
use scraper::Html;

async fn get_content(url: Url) -> Result<(), Error> {
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
    Ok(())
}

/// Read and navidgae the links to endpoint on the left side of the page
fn endpoint_links(document: &Html, base_url: &Url) -> Result<Vec<Url>, Error> {
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
            println!("{url}");
            out.push(url);
        }
    }
    Ok(out)
}

// /// Read the get and post requests
// async fn api_reader(&self, _response: Response, a: ElementRef) -> Result<(), CrablerError> {
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
