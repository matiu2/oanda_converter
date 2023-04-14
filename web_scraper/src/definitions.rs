//! Reads the definitions, eg: https://developer.oanda.com/rest-live-v20/instrument-df/
use crate::{bail, report, Error, Result};
use model::defintion_docs::{Definition, EnumItem, Struct, Value};
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
        .map(read_header)
        .collect::<Result<Vec<Header>>>()?;
    let body_selector =
        scraper::Selector::parse("#content-api-section .definition_body").map_err(Error::from)?;
    let bodies = document
        .select(&body_selector)
        .map(read_body)
        .inspect(|body| println!("Got this body: {body:#?}"))
        .collect::<Result<Vec<Value>>>()?;
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

fn read_body(fragment: ElementRef) -> Result<Value> {
    let enum_selector = scraper::Selector::parse("table.parameter_table").map_err(Error::from)?;
    let struct_selector = scraper::Selector::parse("pre.json_schema").map_err(Error::from)?;
    let enum_fragment = fragment.select(&enum_selector).next();
    let struct_fragment = fragment.select(&struct_selector).next();
    match (enum_fragment, struct_fragment) {
        (Some(_), Some(_)) => bail!("Found both an enum and struct definition in the same body: {fragment:#?} using {enum_selector:#?} and {struct_selector:#?}"),
        (Some(enum_fragment), None) => Ok(Value::Enum(read_enum(enum_fragment)?)),
        (None, Some(struct_fragment)) => Ok(Value::Struct(read_struct(struct_fragment)?)),
        (None, None) => bail!("Unable to find either an enum nor a strcut definition in {fragment:#?} using {enum_selector:#?} and {struct_selector:#?}"),
    }
}

fn read_enum(fragment: ElementRef) -> Result<Vec<EnumItem>> {
    let th_selector = scraper::Selector::parse("th").map_err(Error::from)?;
    let table_headers: Vec<&str> = fragment
        .select(&th_selector)
        .flat_map(|th| th.text())
        .collect();
    let expected = [["Type", "string"], ["Value", "Description"]];
    if !expected
        .iter()
        .any(|expected_option| table_headers.as_slice() == expected_option)
    {
        bail!("Unexpected table headers for enum: got {table_headers:#?} expected: {expected:#?}");
    }
    // Read in the values
    let row_selector = scraper::Selector::parse("tbody tr").map_err(Error::from)?;
    let cell_selector = scraper::Selector::parse("th,td").map_err(Error::from)?;
    let get_row = |row: ElementRef| {
        let mut cells = row.select(&cell_selector);
        let mut next = move || Some(cells.next()?.text().next()?.to_string());
        let value = next()?;
        let description = next()?;
        Some(EnumItem { value, description })
    };
    let rows = fragment.select(&row_selector);
    if rows.clone().take(2).count() != 2 {
        bail!("Enum {} doesn't even have two entries.", fragment.html())
    }
    rows.clone().map(|row| {
         get_row(row).ok_or_else(|| report!("Unable to read value and description from enum table row: {} using {cell_selector:#?}", row.html()))
    })
    .collect()
}

fn read_struct(fragment: ElementRef) -> Result<Struct> {
    todo!()
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
