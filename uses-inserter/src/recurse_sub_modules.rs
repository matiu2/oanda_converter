use crate::{Error, Mod, ModName, Result};
use error_stack::{Report, ResultExt};
use log::debug;
use std::collections::VecDeque;

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
    type Item = Result<Mod<'a>>;

    /// Get's the next rust module recursively, breadth first
    fn next(&mut self) -> Option<Self::Item> {
        // If there's something in the queue, parse it
        let mod_name = self.queue.pop_front()?;
        debug!("getting data for {mod_name}");
        match std::fs::read_to_string(mod_name.file_name())
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Reading {}", mod_name.file_name())))
        {
            Ok(s) => match syn::parse_str(&s)
                .map_err(Report::from)
                .change_context_lazy(|| {
                    Error::new(format!("Parsing contents of {}", mod_name.file_name()))
                }) {
                Ok(contents) => {
                    // Extracts the module names declared in the current rust module, as strings
                    // then turns them into module names relative to the current module
                    // And appends them to our queue
                    self.queue
                        .extend(get_mods(&contents).map(|s| mod_name.clone().add_part(s)));
                    Some(Ok(Mod { mod_name, contents }))
                }
                Err(err) => Some(Err(err)),
            },
            Err(err) => Some(Err(err)),
        }
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
    use crate::{Error, Result};
    use error_stack::{Report, ResultExt};
    use pretty_assertions::assert_eq;
    use proc_macro2::TokenStream;
    use quote::quote;

    #[test]
    fn test_get_mods() -> Result<()> {
        let file_name = "../writer/src/lib.rs";
        let s = std::fs::read_to_string(file_name)
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Opening {file_name}")))?;
        let code: TokenStream = syn::parse_str(&s)
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing string {s}")))?;
        let code: syn::File = syn::parse2(code.clone())
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing tokens {code:#?}")))?;
        let mods: Vec<String> = super::get_mods(&code).collect();
        let expected = vec![
            "error",
            "gen_client",
            "gen_definition",
            "gen_endpoint",
            "gen_error",
            "gen_mods",
            "util",
        ];
        assert_eq!(expected, mods);
        Ok(())
    }

    #[test]
    fn test_inline_mod() -> Result<()> {
        let file_name = "../oanda_v2/src/client.rs";
        let s = std::fs::read_to_string(file_name)
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Reading {file_name}")))?;
        let code: TokenStream = syn::parse_str(&s)
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing string {s}")))?;
        let code: syn::File = syn::parse2(code.clone())
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing tokens {code:#?}")))?;
        let mods: Vec<String> = super::get_mods(&code).collect();
        println!("{mods:#?}");
        let expected: Vec<&str> = vec![];
        assert_eq!(expected, mods);
        Ok(())
    }

    #[test]
    fn test_public_mod() -> Result<()> {
        let input = quote! {
            pub mod responses;
        };
        let code: syn::File = syn::parse2(input.clone())
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing tokens {input:#?}")))?;
        let mods: Vec<String> = super::get_mods(&code).collect();
        println!("{mods:#?}");
        let expected: Vec<&str> = vec!["responses"];
        assert_eq!(expected, mods);
        Ok(())
    }

    #[test]
    fn test_endpoints_account() -> Result<()> {
        let file_name = "../oanda_v2/src/endpoints/account.rs";
        let s = std::fs::read_to_string(file_name)
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Opening {file_name}")))?;
        let code: TokenStream = syn::parse_str(&s)
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing string {s}")))?;
        let code: syn::File = syn::parse2(code.clone())
            .map_err(Report::from)
            .change_context_lazy(|| Error::new(format!("Parsing tokens {code:#?}")))?;
        let mods: Vec<String> = super::get_mods(&code).collect();
        let expected = vec!["responses"];
        assert_eq!(expected, mods);
        Ok(())
    }
}
