mod mod_name;
use log::debug;
pub use mod_name::ModName;
use std::collections::{HashMap, VecDeque};

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
/// Don't worry. It doesn't load all the mods into memory; it pops them out one at a time.
pub fn recurse_sub_modules(mod_name: ModName<'_>) -> Box<dyn Iterator<Item = Mod<'_>> + '_> {
    // Open the file
    let file_name = mod_name.file_name();
    let s = std::fs::read_to_string(file_name).unwrap();
    let contents: syn::File = syn::parse_str(&s).unwrap();
    let my_mod = mod_name.clone();
    let sub_mods: Vec<String> = get_mods(&contents).collect();
    debug!("getting data for {mod_name}");
    let once = std::iter::once(Mod { mod_name, contents });
    let sub_mods = sub_mods
        .into_iter()
        .map(move |s| my_mod.clone().add_part(s))
        .flat_map(recurse_sub_modules);
    Box::new(once.chain(sub_mods))
}

#[derive(Default, Debug)]
struct RecurseSubModules<'a> {
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

/// Given a mod name, returns the contents of the rust module
fn get_file_contents(mod_name: &ModName<'_>) -> syn::File {
    let file_name = mod_name.file_name();
    let s = std::fs::read_to_string(file_name).unwrap();
    syn::parse_str(&s).unwrap()
}

impl<'a> Iterator for RecurseSubModules<'a> {
    type Item = Mod<'a>;

    /// Get's the next rust module recursively, breadth first
    fn next(&mut self) -> Option<Self::Item> {
        // If there's something in the queue, parse it
        let mod_name = self.queue.pop_front()?;
        debug!("getting data for {mod_name}");
        let contents = get_file_contents(&mod_name);
        // Extracts the module names declared in the current rust module, as strings
        // then turns them into module names relative to the current module
        // And appends them to our queue
        self.queue
            .extend(get_mods(&contents).map(|s| mod_name.clone().add_part(s)));
        Some(Mod { mod_name, contents })
    }
}

pub fn mod_info_recursive(mod_name: ModName<'_>) -> Vec<ModInfo> {
    recurse_sub_modules(mod_name)
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
    todo!()
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
    use log::debug;
    use proc_macro2::TokenStream;

    use crate::ModName;

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

    #[test]
    fn test_recursion() {
        pretty_env_logger::init();
        let base = ModName::new("../oanda_v2/src").add_part("lib");
        for module in super::recurse_sub_modules(base) {
            debug!("Using module {}", module.mod_name);
        }
    }

    #[test]
    fn test_recursion2() {
        pretty_env_logger::init();
        let base = ModName::new("../oanda_v2/src").add_part("lib");
        for module in super::RecurseSubModules::new(base) {
            debug!("Using module {}", module.mod_name);
        }
    }
}
