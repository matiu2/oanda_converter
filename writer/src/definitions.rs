use crate::Result;
use codegen::Scope;
use model::defintion_docs::{Definition, EnumItem, Field, Struct, Value};
use std::path::Path;

/// Returns the module name so you can import it
pub fn create_definition(dir: &Path, definition: &[Definition]) -> Result<Option<String>> {
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

/// Generates code for a field in a struct
fn field(scope: &mut codegen::Struct, field: model::defintion_docs::Field) -> &mut codegen::Field {
    let basic_type_name = field.type_name.as_str();
    let type_name = match (field.is_array, field.required) {
        (true, true) => format!("Option<Vec<{basic_type_name}>>"),
        (true, false) => format!("Vec<{basic_type_name}>"),
        (false, true) => format!("Option<{basic_type_name}>"),
        (false, false) => format!("{basic_type_name}"),
    };
    scope.new_field(field.name, type_name).doc(field.doc_string)
    // TODO: Work out how to deal with the field.default value. Perhaps generate a new() that handles it.
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

    #[test]
    fn test_field() {
        let mut scope = codegen::Scope::new();
        let r#struct = scope.new_struct("SomeStruct");
        for (name, (is_array, required)) in
            ('a'..).zip([(false, false), (false, true), (true, false), (true, true)])
        {
            super::field(
                r#struct,
                model::defintion_docs::Field {
                    name: format!("{name}"),
                    type_name: "SomeType".to_string(),
                    doc_string: "Very nice docs".to_string(),
                    is_array,
                    default: Some("Amazing".to_string()),
                    required,
                },
            );
        }
        let code = scope.to_string();
        crate::print_code(&code);
        assert_eq!(
            code,
            indoc!(
                "
struct SomeStruct {
    /// Very nice docs
    a: SomeType,
    /// Very nice docs
    b: Option<SomeType>,
    /// Very nice docs
    c: Vec<SomeType>,
    /// Very nice docs
    d: Option<Vec<SomeType>>,
}"
            )
        );
    }
}
