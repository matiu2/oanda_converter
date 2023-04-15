use model::defintion_docs::{Field, Struct};
use pest::Parser;
use pest_derive::Parser;

use crate::{bail, Error, Result};
use error_stack::{IntoReport, ResultExt};

#[derive(Parser)]
#[grammar = "src/definitions/struct.pest"]
struct FieldsParser;

/// Once a struct definition text is downloaded, it parses the pseudo
/// json from the oanda docs and gives you a nice rust representation
pub fn parse_struct(input: &str) -> Result<Struct> {
    let mut fields = Vec::new();
    let pairs = FieldsParser::parse(Rule::data, input)
        .into_report()
        .change_context(Error::default())?;

    for pair in pairs.into_iter() {
        let (line, col) = pair.line_col();
        let span = pair.as_span().as_str();
        match pair.as_rule() {
            Rule::field => {
                let inner = pair.into_inner();
                let doc_string_lines: Vec<&str> = inner
                    .clone()
                    .take_while(|pair| pair.as_rule() == Rule::doc_string_line)
                    .map(|pair| pair.as_str())
                    .collect();
                let doc_string = doc_string_lines.join(" ");
                let mut rest = inner.skip(doc_string_lines.len());
                let Some(name) = rest.next().map(|pair| pair.as_str().to_string()) else 
                    { bail!("No field_name found while parsing field starting at {line}:{col} ({span}) in struct data: {input}")};
                let deprecated = rest.clone().any(|pair| pair.as_rule() == Rule::deprecated);
                if deprecated {
                    continue;
                }
                let default = rest
                    .clone()
                    .find(|pair| pair.as_rule() == Rule::default)
                    .map(|pair| pair.as_str().to_string());
                let type_name_normal = rest
                    .clone()
                    .find(|pair| pair.as_rule() == Rule::type_name_normal)
                    .map(|pair| pair.as_str().to_string());
                let type_name_array = rest
                    .clone()
                    .find(|pair| pair.as_rule() == Rule::type_name_array)
                    .map(|pair| pair.as_str().to_string());
                let (type_name, is_array) = match (type_name_normal, type_name_array) {
                    (None, None) => bail!("No type_name found in field {span} at {line}:{col} in {input}"),
                    (None, Some(type_name)) => (type_name, true),
                    (Some(type_name), None) => (type_name, false),
                    (Some(_), Some(_)) => bail!("Unreachable code. Somehow a field contains both array and not array {span} at {line}:{col} in {input}"),
                };
                fields.push(Field {
                    doc_string,
                    name,
                    type_name,
                    is_array,
                    default,
                });
            }
            // End of input
            Rule::EOI => (),
            // Anything else should never happen, but we'd like to know about it still
            other => {
                bail!("Unexpected parser token {other:?} found at {line}:{col} ({span}) in {input}")
            }
        }
    }
    Ok(Struct { fields })
}

#[cfg(test)]
mod unit_tests {

    #[test]
    fn parse_field() {
        let input = r#"{
    # 
    # The order book’s instrument
    # 
    instrument : (InstrumentName),

    # 
    # The time when the order book snapshot was created.
    # 
    time : (DateTime),

    # 
    # The price (midpoint) for the order book’s instrument at the time of the
    # order book snapshot
    # 
    price : (PriceValue),

    # 
    # Representation of how many units of an Instrument are available to be
    # traded by an Order depending on its positionFill option.
    # 
    # 
    # Deprecated: Will be removed in a future API update.
    # 
    unitsAvailable : (UnitsAvailable, deprecated)

    # 
    # The price width for each bucket. Each bucket covers the price range from
    # the bucket’s price to the bucket’s price + bucketWidth.
    # 
    bucketWidth : (PriceValue),

    # 
    # The string “PRICE”. Used to identify the a Price object when found in a
    # stream.
    # 
    type : (string, default=PRICE),

    # 
    # The partitioned order book, divided into buckets using a default bucket
    # width. These buckets are only provided for price ranges which actually
    # contain order or position data.
    # 
    buckets : (Array[OrderBookBucket])
}"#;
        let got = super::parse_struct(input).unwrap();
        let get_field = |name| got.fields.iter().find(|field| field.name == name).unwrap();
        let price = get_field("price");
        assert_eq!(&price.doc_string, "The price (midpoint) for the order book’s instrument at the time of the order book snapshot");
        assert_eq!(&price.type_name, "PriceValue");
        assert_eq!(price.is_array, false);
        assert_eq!(price.default, None);
        // See if it can read an array OK
        let buckets = get_field("buckets");
        assert_eq!(&buckets.type_name, "OrderBookBucket");
        assert_eq!(buckets.is_array, true);
        assert_eq!(buckets.default, None);
        assert_eq!(&buckets.doc_string, "The partitioned order book, divided into buckets using a default bucket width. These buckets are only provided for price ranges which actually contain order or position data.");
        // Make sure it ignores the deprecated field
        assert!(!got
            .fields
            .iter()
            .any(|field| field.name == "unitsAvalaible"));
        // Read a default value
        let type_field = get_field("type");
        assert_eq!(&type_field.type_name, "string");
        assert_eq!(type_field.is_array, false);
        assert_eq!(type_field.default.as_deref(), Some("PRICE"));
        assert_eq!(
            &type_field.doc_string,
            "The string “PRICE”. Used to identify the a Price object when found in a stream."
        );
    }
}
