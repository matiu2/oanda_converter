//! Generates error.rs for oanda_v2
use crate::{bail, error::Result, Error};
use error_stack::ResultExt;
use model::definition_docs::Row;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use utils::pretty_doc_string;

/// Returns a doc_string if there is one
fn doc_string(row: &Row) -> String {
    match row {
        Row::ValueDescription { description, .. } => description.to_string(),
        Row::FormattedExample {
            format: fmt,
            example,
            ..
        } => format!("Format: {fmt}\n\nExample: {example}"),
        Row::Example { example, .. } => example.to_string(),
        Row::Format { format, .. } => format.to_string(),
        Row::JustType { .. } => String::default(),
    }
}

/// Returns the type name of the field to be generate by this row (that
/// comes from the HTML table in the Oanda documentation)
fn type_name(row: &Row) -> Ident {
    let type_name = match row {
        Row::ValueDescription { .. } => "string",
        Row::FormattedExample { r#type, .. } => r#type,
        Row::Example { r#type, .. } => r#type,
        Row::Format { r#type, .. } => r#type,
        Row::JustType { r#type } => r#type,
    };
    let type_name = change_case::pascal_case(type_name);
    Ident::new(&type_name, proc_macro2::Span::call_site())
}

/// Generates the rust code for a table row from the documentation
/// Where there is only one row in the documentation table
pub fn gen_single_row(row: &Row, name: &str, struct_doc_string: &str) -> Result<TokenStream> {
    let struct_name = Ident::new(name, proc_macro2::Span::call_site());
    let field_doc_string = doc_string(row);
    let type_name = type_name(row);
    let doc_string = format!("{struct_doc_string}\n\n{field_doc_string}");
    let doc_string =
        pretty_doc_string(&doc_string).change_context_lazy(|| Error::new("Creating doc string"))?;
    Ok(quote! {
        #(#doc_string)*
        struct #struct_name(#type_name);

        impl std::ops::Deref for #struct_name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.0.as_str()
            }
        }
    })
}

/// Generates an enum from an HTML table that has more than one row
pub fn gen_rows(rows: &[Row], enum_name: &str, enum_doc_string: &str) -> Result<TokenStream> {
    let enum_name = Ident::new(enum_name, proc_macro2::Span::call_site());
    let doc_string = pretty_doc_string(enum_doc_string).change_context_lazy(Error::default)?;
    let enum_variants = rows
        .iter()
        .map(|row: &Row| match row {
            Row::ValueDescription { value, description } => Ok({
                let value = change_case::pascal_case(value);
                let variant_name = Ident::new(&value, proc_macro2::Span::call_site());
                let doc_string = pretty_doc_string(description)
                    .change_context_lazy(|| Error::new("Creating doc string"))?;
                quote! {
                    #(#doc_string)*
                    #variant_name,
                }
            }),
            _ => bail!("Unexpected row type in multi-row table: {row:#?}"),
        })
        .collect::<Result<Vec<TokenStream>>>()?;
    Ok(quote! {
        #(#doc_string)*
        #[derive(Deserialize, Serialize)]
        #[serde(rename_all="SCREAMING_SNAKE_CASE")]
        pub enum #enum_name {
            #(#enum_variants)*
        }

    })
}

#[cfg(test)]
mod test {
    use error_stack::ResultExt;
    use model::definition_docs::Row;
    use utils::stream_to_string;

    use crate::Error;

    #[test]
    fn test_gen_single_row() -> crate::error::Result<()> {
        let input = Row::ValueDescription {
            value: "some_field".to_string(),
            description: "this is a very important field".to_string(),
        };
        let tokens = super::gen_single_row(&input, "SuperStruct", "This is a fine struct")?;
        let code = stream_to_string(&tokens).change_context_lazy(Error::default)?;
        println!("{code}");
        assert!(code.contains("SuperStruct"));
        Ok(())
    }

    #[test]
    fn test_gen_rows() -> crate::error::Result<()> {
        let input = vec![
            Row::ValueDescription {
                value: "Monday".to_string(),
                description: "Monday".to_string(),
            },
            Row::ValueDescription {
                value: "Tuesday".to_string(),
                description: "Tuesday".to_string(),
            },
            Row::ValueDescription {
                value: "Wednesday".to_string(),
                description: "Wednesday".to_string(),
            },
            Row::ValueDescription {
                value: "Thursday".to_string(),
                description: "Thursday".to_string(),
            },
            Row::ValueDescription {
                value: "Friday".to_string(),
                description: "Friday".to_string(),
            },
            Row::ValueDescription {
                value: "Saturday".to_string(),
                description: "Saturday".to_string(),
            },
            Row::ValueDescription {
                value: "Sunday".to_string(),
                description: "Sunday".to_string(),
            },
        ];
        let tokens = super::gen_rows(input.as_slice(), "SuperEnum", "This is a fine Enum")?;
        let code = stream_to_string(&tokens).change_context_lazy(Error::default)?;
        println!("{code}");
        assert!(code.contains("SuperEnum"));
        Ok(())
    }
}
