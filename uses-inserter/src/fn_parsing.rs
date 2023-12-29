use syn::FnArg;

use crate::fields::get_type_name;

/// Given a syn Fn - returns the type names from both the arguments and the return type
pub fn get_type_names_from_fn(f: &syn::ItemFn) -> Vec<String> {
    let arg_type_names = f.sig.inputs.iter().flat_map(|arg| {
        if let FnArg::Typed(arg) = arg {
            get_type_name(&arg.ty)
        } else {
            vec![]
        }
    });
    let out_type = match &f.sig.output {
        syn::ReturnType::Type(_, ty) => get_type_name(ty),
        _ => vec![],
    };
    arg_type_names.chain(out_type).collect()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use quote::quote;

    #[test]
    fn test_fn_parsing() {
        let input = quote! {
            pub async fn trades(
                &self,
                authorization: String,
                accept_datetime_format: AcceptDatetimeFormat,
                account_id: AccountId,
                ids: ListOf,
                state: TradeStateFilter,
                instrument: InstrumentName,
                count: Integer,
                before_id: TradeId,
            ) -> Result<SomeType> {todo!()}
        };
        let f = syn::parse2(input).unwrap();
        let got = super::get_type_names_from_fn(&f);
        let expected = vec![
            "String",
            "AcceptDatetimeFormat",
            "AccountId",
            "ListOf",
            "TradeStateFilter",
            "InstrumentName",
            "Integer",
            "TradeId",
            "Result",
            "SomeType",
        ];
        assert_eq!(expected, got);
    }
}
