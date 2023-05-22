use crate::{bail, Result};
use codegen::Scope;
use convert_case::{Case, Casing};
use model::defintion_docs::{Definition, EnumItem, Struct, Value};
use std::path::Path;

/// Returns the module name so you can import it
pub fn create_definition(dir: &Path, definition: &[Definition]) -> Result<Option<String>> {
    todo!()
}

/// Generates a rust struct or enum from a schema
fn definition(definition: &Definition, scope: &mut Scope) -> Result<()> {
    match &definition.value {
        Value::Enum(items) => {
            let is_enum_variant = |item: &EnumItem| match item {
                EnumItem::ValueDescription { .. } => true,
                _ => false,
            };
            let Some(is_enum) = items.first().map(is_enum_variant) else { return Ok(()) };
            if is_enum {
                // Generate an enum
                if !items.iter().all(is_enum_variant) {
                    bail!("All items should be enum variants: {definition:#?}");
                }
                let e = scope
                    .new_enum(&definition.name)
                    .doc(&definition.doc_string)
                    .derive("Serialize")
                    .derive("Deserialize")
                    .derive("Debug");
                items.iter().for_each(|field| match field {
                    EnumItem::ValueDescription { value, description } => {
                        e.new_variant(value.to_case(Case::Pascal))
                            .annotation(format!("/// {description}"));
                    }
                    other => unreachable!("{other:#?}"),
                });
            } else {
                // Generate a struct
                let mut doc_string = vec![definition.doc_string.clone()];
                let s = scope
                    .new_struct(&definition.name)
                    .doc(&definition.doc_string);
                for field in items {
                    match field {
                        EnumItem::FormattedExample {
                            r#type,
                            format,
                            example,
                        } => {
                            if r#type != "string" {
                                bail!("We expected all type,formate,example inputs to be type=string: {definition:#?}");
                            }
                            doc_string.push(format!("Format: {format}"));
                            doc_string.push(format!("Example: {example}"));
                            s.doc(&doc_string.join("\n"));
                            s.derive("Deref");
                            s.tuple_field("String");
                        }
                        EnumItem::Example { r#type, example } => todo!(),
                        EnumItem::Format { r#type, format } => todo!(),
                        EnumItem::JustType { r#type } => todo!(),
                        EnumItem::ValueDescription { .. } => {
                            bail!("Unexpted enum variant in struct: {definition:#?}")
                        }
                    }
                }
            }
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
    Ok(())
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
    use crate::{annotate, Result};
    use codegen::Scope;
    use indoc::indoc;
    use model::defintion_docs::Definition;

    #[test]
    fn test_account_id() -> Result<()> {
        let input = indoc! {"
    name: AccountID
    doc_string: The string representation of an Account Identifier.
    value: !Enum
    - !FormattedExample
      type: string
      format: “-“-delimited string with format “{siteID}-{divisionID}-{userID}-{accountNumber}”
      example: 001-011-5838423-001
        "};
        let definition: Definition = annotate!(serde_yaml::from_str(input), "Parsing yaml")?;

        let mut scope = Scope::new();
        super::definition(&definition, &mut scope)?;
        crate::print_code(&scope.to_string());
        Ok(())
    }

    #[test]
    fn test_guaranteed_sl_order_mode() -> Result<()> {
        let input = indoc!("
    name: GuaranteedStopLossOrderMode
    doc_string: The overall behaviour of the Account regarding guaranteed Stop Loss Orders.
    value: !Enum
    - !ValueDescription
      value: DISABLED
      description: The Account is not permitted to create guaranteed Stop Loss Orders.
    - !ValueDescription
      value: ALLOWED
      description: The Account is able, but not required to have guaranteed Stop Loss Orders for open Trades.
    - !ValueDescription
      value: REQUIRED
      description: The Account is required to have guaranteed Stop Loss Orders for all open Trades.
            ");
        let definition: Definition = annotate!(serde_yaml::from_str(input), "Parsing yaml")?;
        let mut scope = Scope::new();
        super::definition(&definition, &mut scope)?;
        let code = scope.to_string();
        crate::print_code(&code);
        assert_eq!(
            code,
            indoc!(
                "
/// The overall behaviour of the Account regarding guaranteed Stop Loss Orders.
#[derive(Serialize, Deserialize, Debug)]
enum GuaranteedStopLossOrderMode {
    /// The Account is not permitted to create guaranteed Stop Loss Orders.
    Disabled,
    /// The Account is able, but not required to have guaranteed Stop Loss Orders for open Trades.
    Allowed,
    /// The Account is required to have guaranteed Stop Loss Orders for all open Trades.
    Required,
}"
            )
        );
        Ok(())
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
