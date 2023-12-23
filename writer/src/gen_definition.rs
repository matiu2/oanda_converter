//! Generates error.rs for oanda_v2
use self::{
    gen_row::{gen_rows, gen_single_row},
    gen_struct::gen_typed_string,
};
use crate::{error::Result, state::StateGuard};
pub use gen_struct::gen_struct;
use model::{definition_docs::Value, Definition};
use proc_macro2::TokenStream;

mod gen_row;
mod gen_struct;

pub fn gen_definition(
    Definition {
        name,
        doc_string,
        value,
    }: &Definition,
    state: StateGuard,
) -> Result<TokenStream> {
    match value {
        Value::Table(rows) => match rows.as_slice() {
            [row] => gen_single_row(row, name, doc_string, &mut state),
            rows => gen_rows(rows, name, doc_string, &mut state),
        },
        Value::Struct(s) => gen_struct(s, name, &mut state),
        Value::Empty => gen_typed_string(name, &mut state),
    }
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
        let code = crate::util::stream_to_string(&tokens)?;
        println!("{code}");
        Ok(())
    }
}
