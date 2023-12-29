use crate::fields::get_field_type_names;

/// Given a rust struct that we're parsing, return all the field type names used
pub fn get_struct_field_types(s: &syn::ItemStruct) -> Vec<String> {
    get_field_type_names(&s.fields)
}
