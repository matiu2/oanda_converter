use crate::{bail, Error, Result};
use codegen::Scope;
use convert_case::{Case, Casing};
use model::defintion_docs::{Definition, EnumItem, Struct, Value};
use std::path::Path;

/// Returns the module name so you can import it
pub fn create_definitions(dir: &Path, name: &str, definitions: &[Definition]) -> Result<Scope> {
    let mut scope = Scope::new();

    let code = scope.to_string();

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
            let Some(is_enum) = items.first().map(is_enum_variant) else { bail!("Can't have an enum with no variants: {definition:#?}") };
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
                for field in items {
                    match field {
                        EnumItem::FormattedExample {
                            r#type,
                            format,
                            example,
                        } => {
                            if r#type != "string" {
                                bail!("We expected all type,format,example inputs to be type=string: {definition:#?}");
                            }
                            make_string_struct(
                                scope,
                                &definition.name,
                                &definition.doc_string,
                                Some(&format),
                                Some(&example),
                            )
                        }
                        EnumItem::Example { r#type, example } => {
                            if r#type != "string" {
                                bail!("We expected all type,example inputs to be type=string: {definition:#?}");
                            }
                            make_string_struct(
                                scope,
                                &definition.name,
                                &definition.doc_string,
                                None,
                                Some(&example),
                            )
                        }
                        EnumItem::Format { r#type, format } => {
                            if r#type != "string" {
                                bail!("We expected all type,format inputs to be type=string: {definition:#?}");
                            }
                            make_string_struct(
                                scope,
                                &definition.name,
                                &definition.doc_string,
                                Some(&format),
                                None,
                            )
                        }
                        EnumItem::JustType { r#type } => {
                            if r#type != "string" {
                                bail!("We expected all type inputs to be type=string: {definition:#?}");
                            }
                            make_string_struct(
                                scope,
                                &definition.name,
                                &definition.doc_string,
                                None,
                                None,
                            )
                        }
                        EnumItem::ValueDescription { .. } => {
                            bail!("Unexpted enum variant in struct: {definition:#?}")
                        }
                    }
                }
            }
        }
        Value::Struct(Struct { fields }) => {
            let mut s = scope
                .new_struct(&definition.name)
                .doc(&definition.doc_string)
                .derive("Serialize")
                // TODO: Not all structs need Serialize and Deserialize
                .derive("Deserialize")
                .derive("Debug");
            fields.iter().for_each(|f| {
                field(&mut s, f);
            });
        }
        Value::Empty => {}
    };
    Ok(())
}

/// Makes a struct that just wraps a string, eg `struct SomeValue(String)`
fn make_string_struct(
    scope: &mut Scope,
    name: &str,
    doc_base: &str,
    format: Option<&str>,
    example: Option<&str>,
) {
    let mut doc_string = vec![doc_base.to_string()];
    if let Some(format) = format {
        doc_string.push(format!("Format: {format}"));
    }
    if let Some(example) = example {
        doc_string.push(format!("Example: {example}"));
    }
    let doc = doc_string.join("\n");
    let s = make_struct(scope, name, &doc, &["Deref"]);
    s.tuple_field("String");
}

/// Only used by make_string_struct
fn make_struct<'a>(
    scope: &'a mut Scope,
    name: &'a str,
    doc_string: &'a str,
    extra_derives: &'a [&'a str],
) -> &'a mut codegen::Struct {
    let s = scope
        .new_struct(name)
        .derive("Serialize")
        .derive("Deserialize")
        .derive("Debug");
    s.doc(&doc_string);
    extra_derives.into_iter().for_each(|d| {
        s.derive(d);
    });
    s.tuple_field("String");
    s
}

/// Generates code for a field in a struct
fn field<'a>(
    scope: &'a mut codegen::Struct,
    field: &'a model::defintion_docs::Field,
) -> &'a mut codegen::Field {
    let basic_type_name = field.type_name.as_str();
    let type_name = match (field.is_array, field.required) {
        (true, true) => format!("Option<Vec<{basic_type_name}>>"),
        (true, false) => format!("Vec<{basic_type_name}>"),
        (false, true) => format!("Option<{basic_type_name}>"),
        (false, false) => format!("{basic_type_name}"),
    };
    scope
        .new_field(field.name.to_case(Case::Snake), type_name)
        .doc(&field.doc_string)
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
                &model::defintion_docs::Field {
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

    #[test]
    fn test_struct() -> Result<()> {
        let input = indoc!("
    name: HomeConversionFactors
    doc_string: A HomeConversionFactors message contains information used to convert amounts, from an Instrument’s base or quote currency, to the home currency of an Account.
    value: !Struct
      fields:
      - name: gainQuoteHome
        type_name: ConversionFactor
        doc_string: The ConversionFactor in effect for the Account for converting any gains realized in Instrument quote units into units of the Account’s home currency.
        is_array: false
        default: null
        required: false
      - name: lossQuoteHome
        type_name: ConversionFactor
        doc_string: The ConversionFactor in effect for the Account for converting any losses realized in Instrument quote units into units of the Account’s home currency.
        is_array: false
        default: null
        required: false
      - name: gainBaseHome
        type_name: ConversionFactor
        doc_string: The ConversionFactor in effect for the Account for converting any gains realized in Instrument base units into units of the Account’s home currency.
        is_array: false
        default: null
        required: false
      - name: lossBaseHome
        type_name: ConversionFactor
        doc_string: The ConversionFactor in effect for the Account for converting any losses realized in Instrument base units into units of the Account’s home currency.
        is_array: false
        default: null
        required: false");
        let definition: Definition = annotate!(serde_yaml::from_str(input), "Parsing yaml")?;
        let mut scope = Scope::new();
        super::definition(&definition, &mut scope)?;
        let code = scope.to_string();
        crate::print_code(&code);
        assert_eq!(code, indoc!("
/// A HomeConversionFactors message contains information used to convert amounts, from an Instrument’s base or quote currency, to the home currency of an Account.
#[derive(Serialize, Deserialize, Debug)]
struct HomeConversionFactors {
    /// The ConversionFactor in effect for the Account for converting any gains realized in Instrument quote units into units of the Account’s home currency.
    gain_quote_home: ConversionFactor,
    /// The ConversionFactor in effect for the Account for converting any losses realized in Instrument quote units into units of the Account’s home currency.
    loss_quote_home: ConversionFactor,
    /// The ConversionFactor in effect for the Account for converting any gains realized in Instrument base units into units of the Account’s home currency.
    gain_base_home: ConversionFactor,
    /// The ConversionFactor in effect for the Account for converting any losses realized in Instrument base units into units of the Account’s home currency.
    loss_base_home: ConversionFactor,
}"));
        Ok(())
    }
}
