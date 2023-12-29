mod enum_parsing;
mod fields;
mod fn_parsing;
mod impl_parsing;
mod mod_name;
mod recurse_sub_modules;
mod struct_parsing;

use crate::{
    enum_parsing::get_types_from_enum, impl_parsing::get_type_names_from_impl,
    struct_parsing::get_struct_field_types,
};
use fn_parsing::get_type_names_from_fn;
use itertools::Itertools;
pub use mod_name::ModName;
use proc_macro2::{Ident, Span};
use quote::quote;
use recurse_sub_modules::recurse_sub_modules;
use std::collections::HashMap;
use utils::stream_to_file;

/// Represents a rust module. It's file/mod name + its contents
pub struct Mod<'a> {
    pub mod_name: ModName<'a>,
    pub contents: syn::File,
}

impl<'a> std::fmt::Debug for Mod<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mod: {}", &self.mod_name)
    }
}

/// A rust module and all the types it declares and all the types it requires
#[derive(Debug)]
pub struct ModInfo<'a> {
    pub module: Mod<'a>,
    pub declares: Vec<String>,
    pub requires: Vec<String>,
}

/// Given a rust module address, recursively collects all type declarations and type usages (requirements).
/// The ultimate goal is to insert the use statements at the top of each module so it'll compile
pub fn mod_info_recursive(start: ModName<'_>) -> Vec<ModInfo> {
    recurse_sub_modules(start)
        .map(|m| {
            let declares = collect_declarations(&m.contents);
            let requires = collect_requirements(&m.contents);
            ModInfo {
                module: m,
                declares,
                requires,
            }
        })
        .collect()
}

/// Inserts the uses clauses at the top of each generated source module
pub fn insert_uses_clauses(start: ModName<'_>) {
    let info = mod_info_recursive(start);
    // Make a hashmap of which module declares which type
    let module_by_decl_type: HashMap<&str, &ModName> = info
        .iter()
        .flat_map(|i| i.declares.iter().map(|d| (d.as_str(), &i.module.mod_name)))
        .collect();
    // For each module, create the uses clause and insert it in the top of the file
    for m in &info {
        let uses = m
            .requires
            .iter()
            .map(|r| {
                module_by_decl_type
                    .get(r.as_str())
                    .into_iter()
                    .map(|mod_name| Ident::new(&mod_name.mod_name(), Span::call_site()))
                    .collect_vec()
            })
            .filter(|mods| !mods.is_empty())
            .map(|m| quote! {use #(#m*)::*})
            .collect_vec();
        let contents = std::fs::read_to_string(m.module.mod_name.file_name()).unwrap();
        let contents: syn::File = syn::parse_str(&contents).unwrap();
        let new_contents = quote!(
            #(#uses);*
            #contents
        );
        stream_to_file(new_contents, m.module.mod_name.file_name().as_str()).unwrap();
    }
}

/// Collects all the types that this module needs to import
fn collect_requirements(contents: &syn::File) -> Vec<String> {
    use syn::Item::{Enum, Fn, Impl, Struct};
    contents
        .items
        .iter()
        .flat_map(|i| match i {
            Struct(s) => get_struct_field_types(s),
            Enum(e) => get_types_from_enum(e),
            Impl(i) => get_type_names_from_impl(i),
            Fn(f) => get_type_names_from_fn(f),
            _ => vec![],
        })
        .collect()
}

/// Collects all the struct and enum declarations from a TokenStream
pub fn collect_declarations(code: &syn::File) -> Vec<String> {
    use syn::Item::{Enum, Struct};
    code.items
        .iter()
        .flat_map(|i| match i {
            Struct(s) => Some(s.ident.to_string()),
            Enum(e) => Some(e.ident.to_string()),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod test {
    use log::debug;
    use proc_macro2::TokenStream;

    use crate::ModName;

    #[test]
    fn test_collect_declarations() {
        let s =
            std::fs::read_to_string("../oanda_v2/src/endpoints/account/responses/instruments.rs")
                .unwrap();
        let code: TokenStream = syn::parse_str(&s).unwrap();
        let code: syn::File = syn::parse2(code).unwrap();
        let declarations = super::collect_declarations(&code);
        assert_eq!(vec!["Instruments200".to_string()], declarations);
    }

    #[test]
    fn test_recursion() {
        pretty_env_logger::try_init().ok();
        let base = ModName::new("../oanda_v2/src").add_part("lib");
        for module in super::recurse_sub_modules(base) {
            debug!("Using module {}", module.mod_name);
        }
    }
}
