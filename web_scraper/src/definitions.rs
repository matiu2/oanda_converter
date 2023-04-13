//! Reads the definitions, eg: https://developer.oanda.com/rest-live-v20/instrument-df/
use crate::{bail, Error, Result};
use model::defintion_docs::Definition;
use scraper::{ElementRef, Html};

#[derive(Debug)]
struct Header {
    type_name: String,
    description: String,
}

fn get_definitions(document: &Html) -> Result<Vec<Definition>> {
    let header_selector =
        scraper::Selector::parse("#content-api-section .endpoint_header").map_err(Error::from)?;
    let headers = document
        .select(&header_selector)
        .map(|fragment| read_header(fragment))
        .collect::<Result<Vec<Header>>>()?;
    println!("headers: {headers:#?}");
    todo!()
}

fn read_header(fragment: ElementRef) -> Result<Header> {
    let type_name_selector = scraper::Selector::parse(".method").map_err(Error::from)?;
    let description_selector = scraper::Selector::parse(".definition").map_err(Error::from)?;
    let get_text = |selector| Some(fragment.select(selector).next()?.text().next()?.to_string());
    let Some(type_name) = get_text(&type_name_selector) else { bail!("Unable to read type_name from {fragment:#?} using {type_name_selector:#?}")};
    let Some(description) = get_text(&description_selector) else { bail!("Unable to read description from {fragment:#?} using {description_selector:#?}")};
    Ok(Header {
        type_name,
        description,
    })
}

#[cfg(test)]
mod test {
    use crate::{Error, Result};
    use error_stack::{IntoReport, ResultExt};
    use scraper::Html;

    #[tokio::test]
    async fn read_definitions() -> Result<()> {
        let url = reqwest::Url::parse("https://developer.oanda.com/rest-live-v20/instrument-df/")
            .into_report()
            .change_context(Error::default())?;
        let html = reqwest::get(url.clone())
            .await
            .into_report()
            .change_context(Error::default())?
            .text()
            .await
            .into_report()
            .change_context(Error::default())?;
        let document = Html::parse_document(&html);
        let definitions = super::get_definitions(&document)?;
        todo!()
    }
}
