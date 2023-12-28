mod mod_name;
mod recurse_sub_modules;
mod struct_parsing;
pub use mod_name::ModName;
use recurse_sub_modules::recurse_sub_modules;

use crate::struct_parsing::get_field_types;

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

/// Collects all the types that this module needs to import
fn collect_requirements(contents: &syn::File) -> Vec<String> {
    use syn::Item::{Enum, Struct};
    contents
        .items
        .iter()
        .flat_map(|i| match i {
            Struct(s) => get_field_types(s),
            Enum(e) => todo!(),
            syn::Item::Impl(i) => todo!(),
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
