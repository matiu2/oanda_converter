use crate::fn_parsing::get_type_names_from_fn_sig;

pub fn get_type_names_from_impl(i: &syn::ItemImpl) -> Vec<String> {
    i.items
        .iter()
        .flat_map(|i| match i {
            // syn::ImplItem::Const(_) => "syn::ImplItem::Const(_) ".to_string(),
            syn::ImplItem::Fn(f) => get_type_names_from_fn_sig(&f.sig),
            // syn::ImplItem::Type(_) => "syn::ImplItem::Type(_) ".to_string(),
            // syn::ImplItem::Macro(_) => "syn::ImplItem::Macro(_) ".to_string(),
            // syn::ImplItem::Verbatim(_) => "syn::ImplItem::Verbatim(_) ".to_string(),
            _ => vec![],
        })
        .collect()
}

#[cfg(test)]
mod test {
    use quote::quote;

    #[test]
    fn test_get_type_names_from_impl() {
        let input = quote! {
            impl<'a> Account<'a> { pub async fn some_func(&self, authorization: AuthType) -> Result<OutType> {todo!()} }
        };
        let input: syn::ItemImpl = syn::parse2(input).unwrap();
        let got = super::get_type_names_from_impl(&input);
        let expected = vec!["AuthType", "Result", "OutType"];
        assert_eq!(&expected, &got);
    }
}
