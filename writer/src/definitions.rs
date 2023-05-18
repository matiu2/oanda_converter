use crate::Result;
use codegen::Scope;
use model::defintion_docs::{Definition, EnumItem, Struct, Value};
use std::path::Path;

pub fn create_definition(dir: &Path, definition: &[Definition]) -> Result<()> {
    todo!()
}

/// Generates a rust struct or enum from a schema
fn definition(definition: &Definition, scope: &mut Scope) {
    match &definition.value {
        Value::Enum(items) => {
            let mut doc_string = vec![definition.doc_string.clone()];
            let s = scope.new_struct(&definition.name);
            items.iter().for_each(|field| match field {
                EnumItem::ValueDescription { value, description } => todo!(),
                EnumItem::FormattedExample {
                    r#type,
                    format,
                    example,
                } => {
                    doc_string.push(format!("Format: {format}"));
                    doc_string.push(format!("Example: {example}"));
                    assert_eq!(r#type, "string");
                    s.tuple_field("String");
                }
                EnumItem::Example { r#type, example } => todo!(),
                EnumItem::Format { r#type, format } => todo!(),
                EnumItem::JustType { r#type } => todo!(),
            });
            s.derive("Deref");
            s.doc(&doc_string.join("\n"));
        }
        Value::Struct(Struct { fields }) => {
            let s = scope
                .new_struct(&definition.name)
                .doc(&definition.doc_string);
            fields.iter().for_each(|field| {
                s.new_field(&field.name, &field.type_name);
            });
        }
        Value::Empty => {}
    }
}

#[cfg(test)]
mod tests {
    use codegen::Scope;
    use indoc::indoc;
    use model::defintion_docs::Definition;

    #[test]
    fn test_account_id() {
        let input = indoc! {"
    name: AccountID
    doc_string: The string representation of an Account Identifier.
    value: !Enum
    - !FormattedExample
      type: string
      format: “-“-delimited string with format “{siteID}-{divisionID}-{userID}-{accountNumber}”
      example: 001-011-5838423-001
        "};
        let definition: Definition = serde_yaml::from_str(input).unwrap();

        let mut scope = Scope::new();
        super::definition(&definition, &mut scope);
        println!("Code: {}", scope.to_string());
    }
}
