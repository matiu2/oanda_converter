mod mod_name;
use log::debug;
pub use mod_name::ModName;
use std::collections::HashMap;

/// Parses lib.rs (you pass the path to this)
pub fn create_uses_map_recursive(mod_name: &ModName) -> HashMap<String, String> {
    // Open the file
    let file_name = mod_name.file_name();
    debug!("Opening {file_name}");
    let s = std::fs::read_to_string(file_name).unwrap();
    let code: syn::File = syn::parse_str(&s).unwrap();
    let declarations = collect_declarations(&code);
    let mods = get_mods(&code);
    let base_path = mod_name.new_base_path();

    // Map the declarations from the current module
    let map: HashMap<String, String> = declarations
        .into_iter()
        .map(|d| {
            let mod_name = format!("{base_path}::{d}");
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
            // because they'll be new files
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
