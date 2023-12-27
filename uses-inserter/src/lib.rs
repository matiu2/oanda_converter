mod mod_name;
use log::debug;
pub use mod_name::ModName;
use std::collections::HashMap;

/// Represents a rust module. It's file/mod name + its contents
pub struct Mod<'a> {
    pub mod_name: ModName<'a>,
    pub contents: syn::File,
}

/// A rust module and all the types it declares and all the types it requires
pub struct ModInfo<'a> {
    pub module: Mod<'a>,
    pub declares: Vec<String>,
    pub requires: Vec<String>,
}

/// Opens the given rust mod, and recurses down through all the mods declared.
/// Doesn't support the /mod_name/mod.rs format, only /mod_name.rs + /mod_name/sub_mod.rs
pub fn recurse_sub_modules(mod_name: ModName<'_>) -> Box<dyn Iterator<Item = Mod<'_>> + '_> {
    // Open the file
    let file_name = mod_name.file_name();
    let s = std::fs::read_to_string(file_name).unwrap();
    let contents: syn::File = syn::parse_str(&s).unwrap();
    let my_mod = mod_name.clone();
    let sub_mods: Vec<String> = get_mods(&contents).collect();
    let once = std::iter::once(Mod { mod_name, contents });
    let sub_mods = sub_mods
        .into_iter()
        .map(move |s| my_mod.clone().add_part(s))
        .flat_map(recurse_sub_modules);
    Box::new(once.chain(sub_mods))
}

/// Parses lib.rs (you pass the path to this)
pub fn create_uses_map_recursive(mod_name: &ModName) -> HashMap<String, String> {
    // Open the file
    let file_name = mod_name.file_name();
    debug!("Opening {file_name}");
    let s = std::fs::read_to_string(file_name).unwrap();
    let code: syn::File = syn::parse_str(&s).unwrap();
    let declarations = collect_declarations(&code);
    let mods = get_mods(&code);

    // Map the declarations from the current module
    let map: HashMap<String, String> = declarations
        .into_iter()
        .map(|d| {
            let mod_name = mod_name.clone().add_part(&d).mod_name();
            (d, mod_name)
        })
        .collect();

    // Cycle through all the rust mods this file creates recursively
    // and collect all of their declarations
    mods.map(|m| mod_name.clone().add_part(m))
        .inspect(|m| debug!("About to recurse into {mod}", mod = m.file_name()))
        .map(|m| create_uses_map_recursive(&m))
        .fold(map, |mut acc, input| {
            acc.extend(input);
            acc
        })
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

/// Collects mods from a TokenStream
pub fn get_mods(code: &syn::File) -> impl Iterator<Item = String> + '_ {
    code.items.iter().flat_map(|i| {
        if let syn::Item::Mod(m) = i {
            // We only want to return modules that have no content,
            // because they'll be new filesw
            m.content.is_none().then(|| m.ident.to_string())
        } else {
            None
        }
    })
}

#[cfg(test)]
mod test {
    use proc_macro2::TokenStream;

    #[test]
    fn test_collect_mods() {
        let s = std::fs::read_to_string("../writer/src/lib.rs").unwrap();
        let code: TokenStream = syn::parse_str(&s).unwrap();
        let code: syn::File = syn::parse2(code).unwrap();
        let mods: Vec<String> = super::get_mods(&code).collect();
        let expected = vec![
            "error",
            "gen_client",
            "gen_definition",
            "gen_endpoint",
            "gen_error",
            "gen_lib",
            "util",
        ];
        assert_eq!(expected, mods);
    }

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
    fn test_inline_mod() {
        let s = std::fs::read_to_string("../oanda_v2/src/client.rs").unwrap();
        let code: TokenStream = syn::parse_str(&s).unwrap();
        let code: syn::File = syn::parse2(code).unwrap();
        let mods: Vec<String> = super::get_mods(&code).collect();
        println!("{mods:#?}");
        let expected: Vec<&str> = vec![];
        assert_eq!(expected, mods);
    }
}
