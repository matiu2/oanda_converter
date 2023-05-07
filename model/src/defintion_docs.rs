#[derive(Debug)]
pub struct Definition {
    pub name: String,
    pub doc_string: String,
    pub value: Value,
}

#[derive(Debug)]
pub enum Value {
    Enum(Vec<EnumItem>),
    Struct(Struct),
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnumItem {
    ValueDescription {
        value: String,
        description: String,
    },
    Example {
        r#type: String,
        format: String,
        example: String,
    },
    Format {
        r#type: String,
        format: String,
    },
}

/// A struct definition. Used to create serde types to match oanda json types
#[derive(Debug, Default)]
pub struct Struct {
    // TODO: Maybe add a name and doc_string for the struct itself
    pub fields: Vec<Field>,
}

/// Represents a field in a struct definiton
#[derive(Debug)]
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
