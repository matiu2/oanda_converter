use itertools::Itertools;
use syn::{Field, FieldsNamed, FieldsUnnamed, Path, PathSegment, Type, TypePath};

/// Given some fields, returns all the types of all the fields
pub fn get_field_type_names(fields: &syn::Fields) -> Vec<String> {
    fn get_field_name(f: &Field) -> Option<String> {
        if let Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) = &f.ty
        {
            Some(
                segments
                    .iter()
                    .map(|PathSegment { ident, .. }| ident.to_string())
                    .join(" - "),
            )
        } else {
            None
        }
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

mod test {
    use log::debug;

    #[test]
    fn test_get_field_types_unnamed() {
        let s = quote::quote! {struct MyStruct(Happy, Bun);};
        let s: syn::ItemStruct = syn::parse2(s).unwrap();
        let fields: Vec<String> = super::get_field_type_names(&s.fields);
        for field in &fields {
            debug!("Field: {field}");
        }
        assert_eq!(vec!["Happy", "Bun"], fields);
    }

    #[test]
    fn test_get_field_types_named() {
        let s = quote::quote! {struct MyStruct{field1: Happy, field2: Bun}};
        let s: syn::ItemStruct = syn::parse2(s).unwrap();
        let fields: Vec<String> = super::get_field_type_names(&s.fields);
        for field in &fields {
            debug!("Field: {field}");
        }
        assert_eq!(vec!["Happy", "Bun"], fields);
    }
}
