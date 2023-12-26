use std::collections::HashMap;

struct ModName {
    base_path: String,
    parts: Vec<String>,
}

impl ModName {
    pub fn file_name(&self) -> String {
        format!(
            "{base_path}/{parts}.rs",
            base_path = &self.base_path,
            parts = self.parts.join("/")
        )
    }

    /// The part to put after "uses" when you import this module
    pub fn use_name(&self) -> String {
        format!("crate::{parts}", parts = self.parts.join("/"))
    }
}

/// Parses lib.rs (you pass the path to this)
pub fn create_uses_map_recursive(mod_name: &ModName) -> HashMap<String, String> {
    // Open the file
    let s = std::fs::read_to_string(mod_name.file_name()).unwrap();
    let code: syn::File = syn::parse_str(&s).unwrap();
    let declarations = collect_declarations(&code);
    let mods = collect_mods(&code);

    todo!()
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
pub fn collect_mods(code: &syn::File) -> Vec<String> {
    code.items
        .iter()
        .flat_map(|i| {
            if let (syn::Item::Mod(m)) = i {
                Some(m.ident.to_string())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use proc_macro2::TokenStream;

    #[test]
    fn test_collect_mods() {
        let s = std::fs::read_to_string("../writer/src/lib.rs").unwrap();
        let code: TokenStream = syn::parse_str(&s).unwrap();
        let code: syn::File = syn::parse2(code).unwrap();
        let mods = super::collect_mods(&code);
        println!("{mods:#?}");
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
}
