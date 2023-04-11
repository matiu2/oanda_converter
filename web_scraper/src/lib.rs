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
        pub path: String,
        pub doc_string: String,
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
    let header_selector =
        Selector::parse("#single-column > div.endpoint_header").map_err(Error::from)?;
    let body_selector =
        Selector::parse("#single-column > div.endpoint_body").map_err(Error::from)?;
    // Get the HTTP method and path from the headers
    let http_method_selector = Selector::parse(".method").map_err(Error::from)?;
    let path_selector = Selector::parse(".path").map_err(Error::from)?;
    let doc_string_selector = Selector::parse(".path > p").map_err(Error::from)?;
    #[derive(Debug)]
    struct Header {
        http_method: String,
        path: String,
        doc_string: String
    }
    let headers = document.select(&header_selector).map(|header_fragment| {
        // Is it a header or a body
        // Get the http method fragment
        let Some(http_method) = header_fragment.select(&http_method_selector).next().and_then(|element| element.text().next().map(str::to_string)) else { 
            bail!(Error::new(format!("Couldn't find the http method, with {http_method_selector:#?}: {}", header_fragment.html()[..1000].to_string()))); 
        };
        // Read the path
        let Some(path) = header_fragment.select(&path_selector).next().and_then(|element| element.text().next()).map(str::to_string) else {
            bail!(Error::new(format!("Couldn't find the path with {path_selector:#?}: {}", header_fragment.html())));
        };
        // Read the docstring
        let Some(doc_string) = header_fragment.select(&doc_string_selector).next().and_then(|element| element.text().next()).map(str::to_string) else {
            bail!(Error::new(format!("Couldn't find the doc_string with {doc_string_selector:#?}: {}", header_fragment.html())));
        };
        Ok(Header{http_method, path, doc_string })
    }).collect::<Result<Vec<Header>>>()?;
    println!("{headers:#?}");
    // TODO: Parse the bodies
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
