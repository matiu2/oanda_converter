use async_trait::async_trait;
use crabler_tokio::{CrablerError, Element, Opts, Response, WebScraper};
use url::Url;

#[derive(WebScraper)]
#[on_response(response_handler)]
#[on_html("#resources-api-menu #accordion a", endpoint_links)]
struct Scraper {}

impl Scraper {
    async fn response_handler(&self, response: Response) -> Result<(), CrablerError> {
        if response.url.ends_with(".png") && response.status == 200 {
            println!(
                "Finished downloading {} -> {:?}",
                response.url, response.download_destination
            );
        }
        Ok(())
    }

    /// Links to endpoint on the left side of the page
    async fn endpoint_links(&self, response: Response, a: Element) -> Result<(), CrablerError> {
        if let Some(href) = a.attr("href") {
            // Create absolute URL
            let url = match Url::parse(&href) {
                Ok(url) => url,
                Err(_) => Url::parse(&response.url)
                    .map_err(|err| CrablerError::BodyParsing(format!("{err:#?}")))?
                    .join(&href)
                    .map_err(|err| CrablerError::BodyParsing(format!("{err:#?}")))?,
            };
            println!("{url}");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let scraper = Scraper {};

        // Run scraper starting from given url and using 20 worker threads
        scraper
            .run(
                Opts::default()
                    .with_urls(vec![
                        "https://developer.oanda.com/rest-live-v20/account-ep/",
                    ])
                    .with_threads(2),
            )
            .await
            .unwrap();
    }
}
