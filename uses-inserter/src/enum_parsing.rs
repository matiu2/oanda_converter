use crate::fields::get_field_type_names;

pub fn get_types_from_enum(e: &syn::ItemEnum) -> Vec<String> {
    e.variants
        .iter()
        .flat_map(|e| get_field_type_names(&e.fields))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{Error, Result};
    use error_stack::{Report, ResultExt};
    use quote::quote;

    #[test]
    fn test_get_types_from_enum() -> Result<()> {
        let input = quote! {
            #[derive(Debug)]
            pub enum Error {
                E400(ClientExtensions400),
                E404(ClientExtensions404),
            }
        };
        let e = syn::parse2(input.clone())
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing tokens {input:#?}")))?;
        let got = super::get_types_from_enum(&e);
        assert_eq!(&vec!["ClientExtensions400", "ClientExtensions404",], &got);
        Ok(())
    }
}
