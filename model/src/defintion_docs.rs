use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// A definition from the Definition sections of the docs: eg. https://developer.oanda.com/rest-live-v20/account-df
/// Could contain a struct definition or a string with a format, etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct Definition {
    pub name: String,
    pub doc_string: String,
    pub value: Value,
}

/// The actual data from a json type Definition
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Value {
    Table(Vec<Row>),
    Struct(Struct),
    Empty,
}

/// The docs presented us with a table, and it could be any of these formats
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Row {
    ValueDescription {
        value: String,
        description: String,
    },
    /// There's a type (99.9% chance it's a string) with a format and an example
    FormattedExample {
        r#type: String,
        format: String,
        example: String,
    },
    Example {
        r#type: String,
        example: String,
    },
    Format {
        r#type: String,
        format: String,
    },
    JustType {
        r#type: String,
    },
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Schema {
    Struct(Struct),
    Stream(Stream),
}

/// A struct definition. Used to create serde types to match oanda json types
#[derive(Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Struct {
    pub fields: Vec<Field>,
}

/// A stream definition. Similar to a Struct, but the REST response encoded
/// will be stream of json, with a new object on each line.
/// Each object will be *one of* the `objects`
/// We just give the name of the objects, from that the code generator can
/// Look them up.
#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stream {
    pub objects: HashSet<String>,
}

/// Represents a field in a struct definiton
#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    /// The field name (key in the json)
    pub name: String,
    /// May be a simple type like integer, or a defined type (another struct definition)
    pub type_name: String,
    /// The doc_string to put in the getters and setters for this field
    pub doc_string: String,
    /// True if this field contains an array (vec) of type_name
    pub is_array: bool,
    /// default value if applicable
    pub default: Option<String>,
    /// True if the field is required
    pub required: bool,
}
