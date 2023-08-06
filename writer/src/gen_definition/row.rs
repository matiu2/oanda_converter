//! Generates error.rs for oanda_v2
use crate::{error::Result, pretty_doc_string};
use model::definition_docs::Row;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

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
    let doc_string = pretty_doc_string(&doc_string)?;
    Ok(quote! {
        #(#doc_string)*
        struct #struct_name(#type_name);
    })
}

#[cfg(test)]
mod test {
    use model::definition_docs::Row;

    #[test]
    fn test_gen_single_row() -> crate::error::Result<()> {
        let input = Row::ValueDescription {
            value: "some_field".to_string(),
            description: "this is a very important field".to_string(),
        };
        let tokens = super::gen_single_row(&input, "SuperStruct", "This is a fine struct")?;
        let code = crate::stream_to_string(tokens)?;
        println!("{code}");
        assert!(code.contains("SuperStruct"));
        Ok(())
    }
}
