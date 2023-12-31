use std::{borrow::Cow, hash::Hash};

use itertools::Itertools;

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct ModName<'a> {
    base_path: Cow<'a, str>,
    parts: Vec<Cow<'a, str>>,
}

impl<'a> ModName<'a> {
    pub fn new(base_path: impl ToString) -> Self {
        Self {
            base_path: base_path.to_string().into(),
            parts: Default::default(),
        }
    }

    /// The parts with "lib" and "mod" skipped -- unless mod or lib is at the end
    pub fn parts_for_file_name(&self) -> impl Iterator<Item = &str> + '_ {
        let ln = self.parts.len();
        self.parts
            .iter()
            .map(Cow::as_ref)
            .enumerate()
            .flat_map(move |(i, p)| match (i, p) {
                // If it's the last in the list, don't filter it
                (i, p) if i == ln - 1 => Some(p),
                // Otherwise if it's lib or mod - we don't want it
                (_, "lib") | (_, "mod") => None,
                // Anything else is good
                (_, p) => Some(p),
            })
    }

    /// The parts with "lib" and "mod" skipped
    pub fn parts_for_mod(&self) -> impl Iterator<Item = &str> + '_ {
        self.parts
            .iter()
            .map(Cow::as_ref)
            .filter(|&p| p != "lib" && p != "mod")
    }

    /// Returns the filename of the module including the base_path
    /// eg. ../src/mod1/mod2.rs
    pub fn file_name(&self) -> String {
        format!(
            "{base_path}/{parts}.rs",
            base_path = &self.base_path,
            parts = self.parts_for_file_name().join("/")
        )
    }

    /// The part to put after "uses" when you import this module
    /// eg. crate::mod1::mod2
    pub fn mod_name(&self) -> String {
        format!("crate::{parts}", parts = self.parts_for_mod().join("::"))
    }

    /// The mod parts split by '::'
    /// eg. crate::mod1::mod2 -> [crate, mod1, mod2]
    pub fn mod_parts(&self) -> &[Cow<'_, str>] {
        self.parts.as_slice()
    }

    pub fn add_part(mut self, part: impl ToString) -> ModName<'a> {
        // Take lib off of the end if that was a thing
        self.parts.retain(|p| p != "lib");
        self.parts.push(part.to_string().into());
        self
    }

    /// If you were to recurse deeper, you'd want the base_path of the current mod.
    /// Returns the whole mod in the form of a base_path so you can add new mods to it.
    /// Basically the same as `file_name` but without the `.rs` on the end.
    pub fn new_base_path(&self) -> String {
        format!(
            "{base_path}/{parts}",
            base_path = self.base_path,
            parts = self.parts_for_file_name().join("/")
        )
        .trim_end_matches("/lib")
        .trim_end_matches("/mod")
        .to_string()
    }
}

impl std::fmt::Debug for ModName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let parts = self.parts.iter().map(Deref::deref).collect_vec();
        write!(
            f,
            "base_path: {base_path} parts: {parts:#?}",
            base_path = &self.base_path,
            parts = &self.parts
        )
    }
}

impl std::fmt::Display for ModName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", &self.base_path, self.mod_name())
    }
}

#[cfg(test)]
mod test {
    use super::ModName;
    use std::collections::HashSet;

    fn init() -> ModName<'static> {
        ModName::new("xxx").add_part("a").add_part("b")
    }

    #[test]
    fn test_pop_lib() {
        let a = ModName::new("../oanda_v2").add_part("lib");
        let b = a.add_part("definitions");
        assert!(!b.parts.iter().any(|p| p == "lib"));
    }

    #[test]
    fn test_mod_name() {
        let mod_name = init();
        assert_eq!("xxx/a/b.rs", &mod_name.file_name());
        assert_eq!("crate::a::b", &mod_name.mod_name());
        assert_eq!("xxx/a/b", &mod_name.new_base_path());
        // lib on the end should be included in the filename
        let lib_end = mod_name.add_part("lib");
        assert_eq!("xxx/a/b/lib.rs", &lib_end.file_name());
        // lib on the end should be ignored in the mod name
        assert_eq!("crate::a::b", &lib_end.mod_name());
        // lib on the end should be ignored for the new_base_path
        assert_eq!("xxx/a/b", &lib_end.new_base_path());
    }

    #[test]
    fn test_new_base_path_with_lib_and_mod() {
        let mod_name = init();
        assert_eq!("xxx/a/b", &mod_name.new_base_path());
        // lib shouldn't make a difference to the new base_path
        // eg. if you're looking at src/lib.rs, src is still your base_path
        let mod_name = mod_name.add_part("lib");
        assert_eq!("xxx/a/b", &mod_name.new_base_path());
        // same goes for mod.rs
        // If you're looking at /src/some_mod/mod.rs - the base path is /src/some_mod
        let mod_name = mod_name.add_part("mod");
        assert_eq!("xxx/a/b", &mod_name.new_base_path());
    }

    #[test]
    fn test_hash_set() {
        let base_path = "../oanda_v2";
        let files_to_ignore: HashSet<ModName> = ["host", "error", "lib", "client"]
            .into_iter()
            .map(|s| ModName::new(base_path).add_part(s))
            .collect();
        let host = ModName::new("../oanda_v2").add_part("host");
        assert!(files_to_ignore.contains(&host))
    }
}
