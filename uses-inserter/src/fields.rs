use syn::{Field, FieldsNamed, FieldsUnnamed, Path, PathSegment, Type, TypePath};

/// Takes a syn::Type and returns its type name as a string
pub fn get_type_names(ty: &syn::Type) -> Vec<String> {
    match ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => segments
            .iter()
            .flat_map(|PathSegment { ident, arguments }| {
                let mut args = vec![ident.to_string()];
                args.extend(match arguments {
                    syn::PathArguments::None => vec![],
                    syn::PathArguments::AngleBracketed(args) => args
                        .args
                        .iter()
                        .flat_map(|a| match a {
                            syn::GenericArgument::Type(ty) => get_type_names(ty),
                            _ => vec![],
                        })
                        .collect(),
                    syn::PathArguments::Parenthesized(_) => todo!(),
                });
                args
            })
            .collect(),
        Type::Reference(r) => get_type_names(&r.elem),
        Type::Array(_) => unreachable!("Type::Array(_) "),
        Type::BareFn(_) => unreachable!("Type::BareFn(_) "),
        Type::Group(_) => unreachable!("Type::Group(_) "),
        Type::ImplTrait(_) => vec![],
        Type::Infer(_) => unreachable!("Type::Infer(_) "),
        Type::Macro(_) => unreachable!("Type::Macro(_) "),
        Type::Never(_) => unreachable!("Type::Never(_) "),
        Type::Paren(_) => unreachable!("Type::Paren(_) "),
        Type::Ptr(_) => unreachable!("Type::Ptr(_) "),
        Type::Slice(_) => unreachable!("Type::Slice(_) "),
        Type::TraitObject(_) => unreachable!("Type::TraitObject(_) "),
        Type::Tuple(t) => t.elems.iter().flat_map(get_type_names).collect(),
        Type::Verbatim(_) => unreachable!("Type::Verbatim(_) "),
        _ => unreachable!("_ "),
    }
}

/// Given some fields, returns all the types of all the fields
pub fn get_field_type_names(fields: &syn::Fields) -> Vec<String> {
    fn get_field_name(f: &Field) -> Vec<String> {
        get_type_names(&f.ty)
    }
    match &fields {
        syn::Fields::Named(FieldsNamed { named, .. }) => {
            named.iter().flat_map(get_field_name).collect()
        }
        syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            unnamed.iter().flat_map(get_field_name).collect()
        }
        syn::Fields::Unit => vec![],
    }
}

#[cfg(test)]
mod test {
    use crate::{Error, Result};
    use error_stack::{Report, ResultExt};
    use log::debug;

    #[test]
    fn test_get_field_types_unnamed() -> Result<()> {
        let s = quote::quote! {struct MyStruct(Happy, Bun);};
        let s: syn::ItemStruct = syn::parse2(s.clone())
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing {s:#?}")))?;
        let fields: Vec<String> = super::get_field_type_names(&s.fields);
        for field in &fields {
            debug!("Field: {field}");
        }
        assert_eq!(vec!["Happy", "Bun"], fields);
        Ok(())
    }

    #[test]
    fn test_get_field_types_named() -> Result<()> {
        let s = quote::quote! {struct MyStruct{field1: Happy, field2: Bun}};
        let s: syn::ItemStruct = syn::parse2(s.clone())
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing {s:#?}")))?;
        let fields: Vec<String> = super::get_field_type_names(&s.fields);
        for field in &fields {
            debug!("Field: {field}");
        }
        assert_eq!(vec!["Happy", "Bun"], fields);
        Ok(())
    }

    #[test]
    fn test_get_type_name() -> Result<()> {
        let s = quote::quote! {Result<SomeType>};
        let ty: syn::Type = syn::parse2(s.clone())
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing {s:#?}")))?;
        let fields: Vec<String> = super::get_type_names(&ty);
        for field in &fields {
            debug!("Field: {field}");
        }
        assert_eq!(vec!["Result", "SomeType"], fields);
        Ok(())
    }
}
