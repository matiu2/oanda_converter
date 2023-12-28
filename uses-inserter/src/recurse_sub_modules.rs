use std::collections::VecDeque;

use log::debug;

use crate::{Mod, ModName};

/// Opens the given rust mod, and recurses down through all the mods declared.
/// Doesn't support the /mod_name/mod.rs format, only /mod_name.rs + /mod_name/sub_mod.rs
/// Don't worry. It doesn't load all the mods into memory; it pops them out one at a time.
pub fn recurse_sub_modules(mod_name: ModName<'_>) -> RecurseSubModules {
    RecurseSubModules::new(mod_name)
}

#[derive(Default, Debug)]
pub struct RecurseSubModules<'a> {
    // The queue of modules yet to yield
    queue: VecDeque<ModName<'a>>,
}

impl<'a> RecurseSubModules<'a> {
    fn new(start: ModName<'a>) -> Self {
        Self {
            queue: vec![start].into(),
        }
    }
}

impl<'a> Iterator for RecurseSubModules<'a> {
    type Item = Mod<'a>;

    /// Get's the next rust module recursively, breadth first
    fn next(&mut self) -> Option<Self::Item> {
        // If there's something in the queue, parse it
        let mod_name = self.queue.pop_front()?;
        debug!("getting data for {mod_name}");
        let s = std::fs::read_to_string(mod_name.file_name()).unwrap();
        let contents = syn::parse_str(&s).unwrap();
        // Extracts the module names declared in the current rust module, as strings
        // then turns them into module names relative to the current module
        // And appends them to our queue
        self.queue
            .extend(get_mods(&contents).map(|s| mod_name.clone().add_part(s)));
        Some(Mod { mod_name, contents })
    }
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
    fn test_get_mods() {
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
