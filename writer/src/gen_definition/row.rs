//! Generates error.rs for oanda_v2
use crate::{error::Result, pretty_doc_string};
use model::definition_docs::Row;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub trait EasyRow {
    /// Returns the doc_string part if there is one
    fn doc_string(&self) -> Result<Option<Vec<TokenStream>>>;
}

impl EasyRow for Row {
    /// Returns a doc_string if there is one
    fn doc_string(&self) -> Result<Option<Vec<TokenStream>>> {
        #[allow(unused_assignments)]
        let mut s = String::new();
        let doc_string = match self {
            Row::ValueDescription { description, .. } => description,
            Row::FormattedExample {
                format: fmt,
                example,
                ..
            } => {
                s = format!("Format: {fmt}\n\nExample: {example}");
                &s
            }
            Row::Example { example, .. } => example,
            Row::Format { format, .. } => format,
            Row::JustType { .. } => return Ok(None),
        };
        Ok(Some(pretty_doc_string(doc_string)?))
    }
}

pub fn gen_row(row: &Row) -> Result<TokenStream> {
    match row {
        Row::ValueDescription { value, description } => Ok({
            let value = Ident::new(value, proc_macro2::Span::call_site());
            let doc_string = pretty_doc_string(description)?;
            quote! {
                #(#doc_string)*
                #value: String,
            }
        }),
        Row::FormattedExample {
            r#type,
            format,
            example,
        } => todo!(),
        Row::Example { r#type, example } => todo!(),
        Row::Format { r#type, format } => todo!(),
        Row::JustType { r#type } => todo!(),
    }
}

#[cfg(test)]
mod test {
    use model::definition_docs::Row;
    use quote::quote;

    #[test]
    fn test_gen_row() -> crate::error::Result<()> {
        let input = Row::ValueDescription {
            value: "some_field".to_string(),
            description: "this is a very important field".to_string(),
        };
        let tokens = super::gen_row(&input)?;
        let surround = quote! {
            struct TestStruct {
                #tokens
            }
        };
        let code = crate::stream_to_string(surround)?;
        println!("{code}");
        Ok(())
    }
}
