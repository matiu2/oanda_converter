//! Reads the definitions, eg: https://developer.oanda.com/rest-live-v20/instrument-df/
use error_stack::ResultExt;
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
fn read_body(div: ElementRef) -> Result<Value> {
    let enum_selector = Selector::parse("table.parameter_table").map_err(Error::from)?;
    let struct_selector = Selector::parse("pre.json_schema").map_err(Error::from)?;
    let p_selector = Selector::parse("div > p").map_err(Error::from)?;
    let enum_div = div.select(&enum_selector).next();
    let struct_div = div.select(&struct_selector).next();
    let p = div.select(&p_selector).next();
    // TODO: Read response pseudo json when inside `.endpoint_body` divs
    match (enum_div, struct_div, p) {
        (Some(enum_div), None, None) => Ok(Value::Enum(read_enum(enum_div)?)),
        (None, Some(struct_div), None) => Ok(Value::Struct(read_struct(struct_div)?)),
        (None, None, Some(p)) if p.text().next().map(str::trim) == Some("Implemented by:") => Ok(Value::Empty), 
        (r#enum, r#struct, p) =>  bail!("Found unexpected combination of enum / struct / p in definition body. Expected only one but got enum={} struct={} p={} in html: {}", r#enum.is_some(), r#struct.is_some(), p.is_some(), div.html()),
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
    let row_selector = Selector::parse("tbody tr").map_err(Error::from)?;
    let cell_selector = Selector::parse("th,td").map_err(Error::from)?;
    #[derive(Debug)]
    enum Entry {
        // Used for rotated tables, where the keys are in the first column of every header
        KeyValue { key: String, value: String },
        EnumItem(EnumItem),
    }
    impl Entry {
        /// If this Entry is a key value entry, and the key matches the input_key, returns the value
        fn value(&self, input_key: &str) -> Option<&str> {
            match self {
                Entry::KeyValue { key, value } if key == input_key => Some(value),
                _ => None,
            }
        }
    }
    let get_row = match table_headers.as_slice() {
        ["Type", "string"] => |cells: &[&str]| {
            Ok(match cells {
                [key, value] => Entry::KeyValue { key: key.to_string(), value: value.to_string() },
                other => bail!("Expected cells for `key` and `value`, but got the wrong amount of values {other:#?}")
            })
        },
        ["Value", "Description"] => |cells: &[&str]| {
            Ok(match cells {
                [value, description] => Entry::EnumItem(EnumItem::ValueDescription { value: value.to_string(), description: description.to_string() }),
                other => bail!("Expected cells for `value` and `description`, but got the wrong amount of values {other:#?}")
            })
        },
        ["Type", "Format"] => |cells: &[&str]| {
            Ok(match cells {
                [r#type, format] => Entry::EnumItem(EnumItem::Format { r#type: r#type.to_string(), format: format.to_string() }),
                other => bail!("Expected cells for `type` and `format`, but got the wrong amount of values {other:#?}")
            })
        },
        ["Type", "Format", "Example"] => |cells: &[&str]| {
            Ok(match cells {
                [r#type, format, example] => Entry::EnumItem(EnumItem::Example { r#type: r#type.to_string(), format: format.to_string(), example: example.to_string() }),
                [key, value] => Entry::KeyValue{key: key.to_string(),value: value.to_string()},
                other => bail!("Expected cells for `type`, `format`, and `example` but got the wrong amount of values {other:#?}")
            })
        },
        table_headers => {
            return Err(
                report!("Unexpected table headers for enum: got {table_headers:#?}")
                    .attach_printable(format!("In html fragment: {}", fragment.html())),
            )
        }
    };
    // Read in the values
    let rows = fragment.select(&row_selector);
    if rows.clone().take(2).count() != 2 {
        bail!("Enum {} doesn't even have two entries.", fragment.html())
    }
    let parsed = rows.clone().map(|row| {
        let cells: Vec<&str> = row.select(&cell_selector).flat_map(|cell| cell.text().next()).collect();
        get_row(cells.as_slice())
            .attach_printable_lazy(|| format!("Unable to read value and description from enum table row: {} using {cell_selector:#?}", row.html()))
    })
    .collect::<Result<Vec<Entry>>>()?;
    if let Some(Entry::KeyValue { .. }) = parsed.first() {
        // Handle tables that have been rotated, so that the keys are in the first column, rather than along the top row
        let Some(r#type) = parsed.iter().flat_map(|entry| entry.value("Type")).next().map(str::to_string)
            else { bail!("Expected a type row. {parsed:#?}")};
        let Some(format) = parsed.iter().flat_map(|entry| entry.value("Format")).next().map(str::to_string)
            else { bail!("Expected a format row. {parsed:#?}")};
        let Some(example) = parsed.iter().flat_map(|entry| entry.value("Example")).next().map(str::to_string)
            else { bail!("Expected a format row. {parsed:#?}")};
        Ok(vec![EnumItem::Example {
            r#type,
            format,
            example,
        }])
    } else {
        // This is a normal table with the keys along the top row, so each entry is it's own enum possible value
        parsed
            .into_iter()
            .map(|entry| {
                if let Entry::EnumItem(enum_item) = entry {
                    Ok(enum_item)
                } else {
                    bail!("Expected EnumItem, but got KeyValue: {entry:#?}")
                }
            })
            .collect()
    }
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
pub(crate) fn read_struct(fragment: ElementRef) -> Result<Struct> {
    // Create a selector for <a> tags
    let inner_element_selector = Selector::parse("a,b").map_err(Error::from)?;
    let mut inner_element_texts = fragment
        .select(&inner_element_selector)
        .flat_map(|a| a.text());
    let text = fragment
        .children()
        .map(|child| {
            if let Some(element) = child.value().as_element() {
                match element.name()  {
                    "a" | "b" => {
                        let Some(text) = inner_element_texts.next() else {bail!("Anchor element didn't have any text inside: {element:#?}")};
                        Ok(text)
                    },
                    other => {
                        return Err(report!("Unexpected element ({other}) in definition text: {element:#?}")
                            .attach_printable(format!("fragment html:\n```html\n{}\n```", fragment.html())));
                    }
                }
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
    use model::defintion_docs::{EnumItem, Value};
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
        match s5 {
            EnumItem::ValueDescription { value, description } => {
                assert_eq!(value, "S5");
                assert_eq!(description, "5 second candlesticks, minute alignment");
            }
            other => panic!("Unexpuected EnumItem: {other:#?}"),
        };
        let m = values
            .last()
            .ok_or_else(|| report!("Couldn't get last granularity"))?;
        match m {
            EnumItem::ValueDescription { value, description } => {
                assert_eq!(value, "M");
                assert_eq!(
                    description,
                    "1 month candlesticks, aligned to first day of the month"
                );
            }
            other => panic!("Unexpuected EnumItem: {other:#?}"),
        };
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

    #[test]
    fn test_rotated_tables() -> Result<()> {
        let input = r#"<table class="parameter_table">
<tbody>
<tr>
<th class="pt_15">Type</th>
<td class="pt_85">string</td>
</tr>
<tr>
<th>Format</th>
<td>“-“-delimited string with format “{siteID}-{divisionID}-{userID}-{accountNumber}”</td>
</tr>
<tr>
<th>Example</th>
<td>001-011-5838423-001</td>
</tr>
</tbody>
</table>"#;
        let fragment = Html::parse_fragment(input);
        let got = super::read_enum(fragment.root_element())?;
        assert_eq!(got, vec![EnumItem::Example{r#type: "string".to_string(),
            format:"“-“-delimited string with format “{siteID}-{divisionID}-{userID}-{accountNumber}”".to_string(),
            example: "001-011-5838423-001".to_string() }]);
        Ok(())
    }

    #[test]
    fn test_empty_body() -> Result<()> {
        let input = r#"<div class="definition_body collapse" id="collapse_definition_11">
  <p>Implemented by: 
    <a href="../order-df/#MarketOrderRequest">MarketOrderRequest</a>, 
	<a href="../order-df/#LimitOrderRequest">LimitOrderRequest</a>, 
	<a href="../order-df/#StopOrderRequest">StopOrderRequest</a>, 
	<a href="../order-df/#MarketIfTouchedOrderRequest">MarketIfTouchedOrderRequest</a>, 
	<a href="../order-df/#TakeProfitOrderRequest">TakeProfitOrderRequest</a>, 
	<a href="../order-df/#StopLossOrderRequest">StopLossOrderRequest</a>, 
	<a href="../order-df/#GuaranteedStopLossOrderRequest">GuaranteedStopLossOrderRequest</a>, 
	<a href="../order-df/#TrailingStopLossOrderRequest">TrailingStopLossOrderRequest</a>
  </p>
</div>"#;
        let fragment = Html::parse_fragment(input);
        let got = super::read_body(fragment.root_element())?;
        assert_eq!(got, Value::Empty);
        Ok(())
    }
}
