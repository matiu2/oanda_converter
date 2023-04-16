//! Reads the definitions, eg: https://developer.oanda.com/rest-live-v20/instrument-df/
mod struct_parser;
use crate::{bail, report, Error, Result};
use model::defintion_docs::{Definition, EnumItem, Struct, Value};
use scraper::{ElementRef, Html, Selector};

use self::struct_parser::parse_struct;

#[derive(Debug)]
struct Header {
    type_name: String,
    description: String,
}

/// Retrieves a list of definitions from a OANDA REST
/// API defnitions page (where the URL ends with '-df/')
/// eg. (https://developer.oanda.com/rest-live-v20/instrument-df/).
///
/// The function parses the document and extracts headers and bodies
/// from the definition sections (the main body).  It then zips the
/// headers and bodies together to create a list of `Definition` structs.
///
/// # Arguments
///
/// * `document` - A reference to an `Html` object containing the HTML
/// content of the OANDA REST API definitions page.
///
/// # Returns
///
/// * `Result<Vec<Definition>>` - A result containing a vector of
/// `Definition` structs if successful, or an error if an issue occurred
/// during the scraping process.
///
/// # Errors
///
/// This function may return an error if there is a mismatch between
/// the number of headers and bodies, or if there is an issue parsing
/// the HTML content.
pub fn get_definitions(document: &Html) -> Result<Vec<Definition>> {
    let header_selector =
        Selector::parse("#content-api-section .endpoint_header").map_err(Error::from)?;
    let headers = document
        .select(&header_selector)
        .map(read_header)
        .collect::<Result<Vec<Header>>>()?;
    let body_selector =
        Selector::parse("#content-api-section .definition_body").map_err(Error::from)?;
    let bodies = document
        .select(&body_selector)
        .map(read_body)
        .collect::<Result<Vec<Value>>>()?;
    if headers.len() != bodies.len() {
        bail!(
            "Got a different number of headers ({}) to bodies ({}). I can't zip them",
            headers.len(),
            bodies.len()
        );
    }
    Ok(headers
        .into_iter()
        .zip(bodies.into_iter())
        .map(|(header, value)| Definition {
            name: header.type_name,
            doc_string: header.description,
            value,
        })
        .collect())
}

/// Reads a header block from an HTML fragment containing the `type_name` and `description`
/// of the item being documented.
///
/// # Arguments
///
/// * `fragment` - An `ElementRef` instance representing the HTML fragment to be parsed
///
/// # Returns
///
/// * `Result<Header>` - A `Result` containing a `Header` struct with the `type_name` and
/// `description` if successful, otherwise an `Error` if there are any issues with the CSS selectors
///
/// # Errors
///
/// This function will return an error in the following cases:
/// * The CSS selectors for `type_name` and `description` are not valid
/// * The expected elements are not found in the HTML fragment
fn read_header(fragment: ElementRef) -> Result<Header> {
    let type_name_selector = Selector::parse(".method").map_err(Error::from)?;
    let description_selector =
        Selector::parse("span.path > p, span.definition").map_err(Error::from)?;
    let get_text = |selector| Some(fragment.select(selector).next()?.text().next()?.to_string());
    let Some(type_name) = get_text(&type_name_selector) else { bail!("Unable to read type_name from {} using {type_name_selector:#?}", fragment.html())};
    let Some(description) = get_text(&description_selector) else { bail!("Unable to read description from {} using {description_selector:#?}", fragment.html())};
    Ok(Header {
        type_name,
        description,
    })
}

/// Reads the body of a definition. It may be an enum table or a struct
/// (pseudo json)
///
/// # Errors
///
/// This function will return an error if there are any issues with the
/// css selectors or parsing the pseudo json
fn read_body(fragment: ElementRef) -> Result<Value> {
    let enum_selector = Selector::parse("table.parameter_table").map_err(Error::from)?;
    let struct_selector = Selector::parse("pre.json_schema").map_err(Error::from)?;
    let enum_fragment = fragment.select(&enum_selector).next();
    let struct_fragment = fragment.select(&struct_selector).next();
    // TODO: Read response pseudo json when inside `.endpoint_body` divs
    match (enum_fragment, struct_fragment) {
        (Some(enum_fragment), Some(struct_fragment)) => bail!("Found both an enum and struct inside the same body:\nenum: {}\nstruct: {},\nbody: {}", enum_fragment.html(), struct_fragment.html(), fragment.html()),
        (Some(enum_fragment), None) => Ok(Value::Enum(read_enum(enum_fragment)?)),
        (None, Some(struct_fragment)) => Ok(Value::Struct(read_struct(struct_fragment)?)),
        (None, None) => bail!("Unable to find either an enum nor a strcut definition in {} using {enum_selector:#?} and {struct_selector:#?}", fragment.html()),
    }
}

/// Reads an enum table
///
/// # Arguments
///
/// * `fragment: ElementRef` - An ElementRef object representing the
/// HTML block to be parsed.
///
/// # Result
///
/// Returns a `Result<Vec<EnumItem>>` - A Result containing a vector of
/// EnumItem objects if successful, or an error in case of failures.
///
/// # Errors
///
/// This function will return an error if css selectors aren't working
/// on the given HTML block
fn read_enum(fragment: ElementRef) -> Result<Vec<EnumItem>> {
    let th_selector = Selector::parse("th").map_err(Error::from)?;
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
    let row_selector = Selector::parse("tbody tr").map_err(Error::from)?;
    let cell_selector = Selector::parse("th,td").map_err(Error::from)?;
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

/// CSS selects, converts to text, and parses a pseudo json struct definition
///
/// # Arguments
///
/// * `fragment` - An `ElementRef` containing the HTML fragment to be parsed for the struct definition
///
/// # Result
///
/// * A `Result<Struct>` containing the parsed struct on success, or an error if the parsing fails
///
/// # Errors
///
/// This function will return an error if:
/// * The selector fails to parse
/// * An unexpected element is encountered within the definition text
/// * An anchor element has no text inside it
/// * The definition child node is neither text nor an <a> element
fn read_struct(fragment: ElementRef) -> Result<Struct> {
    // Create a selector for <a> tags
    let a_selector = Selector::parse("a").map_err(Error::from)?;
    let mut anchor_texts = fragment.select(&a_selector).flat_map(|a| a.text());
    let text = fragment
        .children()
        .map(|child| {
            if let Some(element) = child.value().as_element() {
                if element.name() != "a" {
                    bail!("Unexpected element in definition text: {element:#?}");
                }
                let Some(text) = anchor_texts.next() else {bail!("Anchor element didn't have any text inside: {element:#?}")};
                Ok(text)
            } else if let Some(text) = child.value().as_text() {
                Ok(text as &str)
            } else {
                bail!("Defintion child node is not text nor <a>; {child:#?}")
            }
        })
        .collect::<Result<String>>()?;
    parse_struct(&text)
}

#[cfg(test)]
mod test {
    use crate::{bail, report, Error, Result};
    use error_stack::{IntoReport, ResultExt};
    use model::defintion_docs::Value;
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
        let definitions =
            super::get_definitions(&document).attach_printable_lazy(|| format!("URL: {url}"))?;
        // Check the first enum
        let granularity = definitions
            .first()
            .ok_or_else(|| report!("No definitions"))?;
        assert_eq!(&granularity.name, "CandlestickGranularity");
        assert_eq!(&granularity.doc_string, "The granularity of a candlestick");
        let Value::Enum(values) = &granularity.value else {bail!("bad granularity")};
        let s5 = values
            .first()
            .ok_or_else(|| report!("No entries in granularities enum"))?;
        assert_eq!(&s5.value, "S5");
        assert_eq!(&s5.description, "5 second candlesticks, minute alignment");
        let m = values
            .last()
            .ok_or_else(|| report!("Couldn't get last granularity"))?;
        assert_eq!(&m.value, "M");
        assert_eq!(
            &m.description,
            "1 month candlesticks, aligned to first day of the month"
        );
        // Check the first struct
        let candlestick = &definitions[2];
        assert_eq!(&candlestick.name, "Candlestick");
        assert_eq!(&candlestick.doc_string, "The Candlestick representation");
        let Value::Struct(data) = &candlestick.value else { bail!("bad candlestick!") };
        let time = &data.fields[0];
        assert_eq!(time.name, "time");
        // Testing more of the struct is beyond the scope of this test, but can be seen in `struct_parser.rs`
        Ok(())
    }
}
