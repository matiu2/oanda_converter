use syn::FnArg;

use crate::fields::get_type_names;

pub fn get_type_names_from_fn_sig(sig: &syn::Signature) -> Vec<String> {
    let arg_type_names = sig.inputs.iter().flat_map(|arg| {
        if let FnArg::Typed(arg) = arg {
            get_type_names(&arg.ty)
        } else {
            vec![]
        }
    });
    let out_type = match &sig.output {
        syn::ReturnType::Type(_, ty) => get_type_names(ty),
        _ => vec![],
    };
    arg_type_names.chain(out_type).collect()
}

/// Given a syn Fn - returns the type names from both the arguments and the return type
pub fn get_type_names_from_fn(f: &syn::ItemFn) -> Vec<String> {
    get_type_names_from_fn_sig(&f.sig)
}

#[cfg(test)]
mod test {
    use crate::{Error, Result};
    use error_stack::{Report, ResultExt};
    use pretty_assertions::assert_eq;
    use quote::quote;

    #[test]
    fn test_fn_parsing() -> Result<()> {
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
        let f = syn::parse2(input.clone())
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing tokens {input:#?}")))?;
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
        Ok(())
    }
}
