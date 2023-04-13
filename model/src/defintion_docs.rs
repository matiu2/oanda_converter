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

#[derive(Debug)]
pub struct EnumItem {
    pub value: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Struct {
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub type_name: String,
    pub doc_string: String,
}
