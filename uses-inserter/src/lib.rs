mod enum_parsing;
mod error;
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
pub use error::{Error, Result};
use error_stack::{Report, ResultExt};
use fn_parsing::get_type_names_from_fn;
use itertools::Itertools;
pub use mod_name::ModName;
use proc_macro2::{Ident, Span};
use quote::quote;
use recurse_sub_modules::recurse_sub_modules;
use std::collections::{HashMap, HashSet};
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
    pub declares: HashSet<String>,
    pub requires: HashSet<String>,
}

/// Given a rust module address, recursively collects all type declarations and type usages (requirements).
/// The ultimate goal is to insert the use statements at the top of each module so it'll compile
pub fn mod_info_recursive(start: ModName<'_>) -> Result<Vec<ModInfo<'_>>> {
    recurse_sub_modules(start)
        .map(|r| {
            let m = r?;
            let declares = collect_declarations(&m.contents);
            let requires = collect_requirements(&m.contents);
            Ok(ModInfo {
                module: m,
                declares,
                requires,
            })
        })
        .collect()
}

/// Inserts the uses clauses at the top of each generated source module
/// `files_to_ignore` will not be processed
/// `known_sources` can be used to declare types not found under start, but also to override found types.
///                 A good example is: [(DateTime, chrono::DateTime)]
pub fn insert_uses_clauses<'a>(
    start: ModName<'a>,
    files_to_ignore: &HashSet<ModName<'a>>,
    known_sources: &HashMap<&'a str, ModName<'a>>,
) -> Result<()> {
    let info = mod_info_recursive(start)?;
    log::info!("info: {info:#?}");
    // Make a hashmap of which module declares which type
    let module_by_decl_type: HashMap<&str, &ModName> = info
        .iter()
        .flat_map(|i| i.declares.iter().map(|d| (d.as_str(), &i.module.mod_name)))
        .chain(known_sources.iter().map(|(&decl, module)| (decl, module)))
        .collect();
    log::info!("declarations by module: {module_by_decl_type:#?}");
    // Find a list of requires that are not in declared
    let not_provided: HashSet<&str> = info
        .iter()
        .flat_map(|m| m.requires.iter())
        .map(|t| t.as_str())
        .filter(|t| !module_by_decl_type.contains_key(t))
        .collect();
    log::info!("Things I need to be declared outside: {not_provided:#?}");
    // For each module, create the uses clause and insert it in the top of the file
    for m in &info {
        if files_to_ignore.contains(&m.module.mod_name) {
            continue;
        }
        let uses = m
            .requires
            .iter()
            // Skip if we declare the requirement ourself
            .filter(|r| !m.declares.contains(r.as_str()))
            .inspect(|mods| {
                log::info!("Getting: {mods:#?}");
            })
            .map(|r| {
                module_by_decl_type
                    .get(r.as_str())
                    .into_iter()
                    .map(|m| {
                        m.mod_parts()
                            .iter()
                            .map(|part| Ident::new(part, Span::call_site()))
                            .collect_vec()
                    })
                    .map(|mod_name| {
                        let r = Ident::new(r, Span::call_site());
                        quote! { #(#mod_name ::)* #r }
                    })
                    .collect_vec()
            })
            .filter(|mods| !mods.is_empty())
            .map(|m| quote! {use #(#m;)*})
            .inspect(|m| log::info!("Sending: {m}"))
            .collect_vec();
        let contents = std::fs::read_to_string(m.module.mod_name.file_name())
            .map_err(Report::from)
            .change_context_lazy(|| {
                Error::Message(format!("Reading {}", m.module.mod_name.file_name()))
            })?;
        let contents: syn::File = syn::parse_str(&contents)
            .map_err(Report::from)
            .change_context_lazy(|| {
                Error::Message(format!(
                    "Parsing contents of {}",
                    m.module.mod_name.file_name()
                ))
            })?;
        let new_contents = quote!(
            #(#uses)*
            #contents
        );
        stream_to_file(new_contents, m.module.mod_name.file_name().as_str())
            .map_err(Report::from)
            .change_context_lazy(|| {
                Error::Message(format!("Writing {}", m.module.mod_name.file_name()))
            })?;
    }
    Ok(())
}

/// Collects all the types that this module needs to import
fn collect_requirements(contents: &syn::File) -> HashSet<String> {
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
pub fn collect_declarations(code: &syn::File) -> HashSet<String> {
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
    use std::collections::HashSet;

    use crate::{Error, ModName, Result};
    use error_stack::{Report, ResultExt};
    use log::debug;
    use proc_macro2::TokenStream;

    #[test]
    fn test_collect_declarations() -> Result<()> {
        let file_name = "../oanda_v2/src/endpoints/account/responses/instruments.rs";
        let s = std::fs::read_to_string(file_name)
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Reading file {file_name}")))?;
        let code: TokenStream = syn::parse_str(&s)
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing file {file_name}")))?;
        let code: syn::File = syn::parse2(code)
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing tokens from {file_name}")))?;
        let declarations = super::collect_declarations(&code);
        assert_eq!(
            vec!["Instruments200".to_string()]
                .into_iter()
                .collect::<HashSet<String>>(),
            declarations
        );
        Ok(())
    }

    #[test]
    fn test_recursion() -> Result<()> {
        pretty_env_logger::try_init().ok();
        let base = ModName::new("../oanda_v2/src").add_part("lib");
        for res in super::recurse_sub_modules(base) {
            let module = res.attach_printable("Recursing sub_modules")?;
            debug!("Using module {}", module.mod_name);
        }
        Ok(())
    }
}
