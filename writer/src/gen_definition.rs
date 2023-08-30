//! Generates error.rs for oanda_v2
use crate::error::Result;
use model::{definition_docs::Value, Definition};
use proc_macro2::TokenStream;

use self::gen_row::{gen_rows, gen_single_row};

mod gen_row;
mod gen_struct;

pub fn gen_definition(
    Definition {
        name,
        doc_string,
        value,
    }: &Definition,
) -> Result<TokenStream> {
    match value {
        Value::Table(rows) => match rows.as_slice() {
            [row] => gen_single_row(row, name, doc_string),
            rows => gen_rows(rows, name, doc_string),
        },
        Value::Struct(_) => todo!(),
        Value::Empty => todo!(),
    }
    // let name = Ident::new(name, proc_macro2::Span::call_site());
    // let doc_string = pretty_doc_string(doc_string)?;

    // Ok(quote! {

    //     #(#doc_string)*
    //     struct #name {
    //     }

    // })
    // match definition.value {
    //     Value::Table(rows) => {
    //         for row in rows {
    //             match row {
    //                 Row::ValueDescription { value, description } => todo!(),
    //                 Row::FormattedExample {
    //                     r#type,
    //                     format,
    //                     example,
    //                 } => todo!(),
    //                 Row::Example { r#type, example } => todo!(),
    //                 Row::Format { r#type, format } => todo!(),
    //                 Row::JustType { r#type } => todo!(),
    //             }
    //         }
    //     }
    //     Value::Struct(s) => todo!(),
    //     Value::Empty => todo!(),
    // }
}

#[cfg(test)]
mod test {
    use model::{definition_docs::Value, Definition};

    #[test]
    fn test_gen_definition() -> crate::error::Result<()> {
        let input = Definition {
            name: "MyStruct".to_string(),
            doc_string: "This is really useful struct.\nThe docstring has multiple lines."
                .to_string(),
            value: Value::Empty,
        };
        let tokens = super::gen_definition(&input)?;
        let code = crate::stream_to_string(&tokens)?;
        println!("{code}");
        Ok(())
    }
}
