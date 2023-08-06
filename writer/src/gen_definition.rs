//! Generates error.rs for oanda_v2
use crate::error::{Error, Result};
use model::Definition;
use quote::{__private::TokenStream, quote};
use syn::Ident;

use crate::error::IntoReport;

pub fn gen_definition(
    Definition {
        name, doc_string, ..
    }: &Definition,
) -> Result<TokenStream> {
    let name = Ident::new(name, proc_macro2::Span::call_site());
    let doc_string = doc_string
        .lines()
        .map(|line| {
            let line = format!("/// {line}");
            line.parse()
                .map_err(|err| Error::Message(format!("{err:#?}")))
                .annotate_lazy(|| "While quoting docstring line: {line}")
        })
        .collect::<Result<Vec<proc_macro2::TokenStream>>>()?;
    Ok(quote! {

        #(#doc_string)*
        struct #name {
        }

    })
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
        let code = crate::stream_to_string(tokens)?;
        println!("{code}");
        Ok(())
    }
}
