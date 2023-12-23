use model::Definition;

use crate::{Error, Result};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default)]
pub struct State {
    mods: VecDeque<String>,
    struct_to_mod: HashMap<String, String>,
}

pub trait ModName {
    fn mod_name(&self) -> &str;
}

impl ModName for Definition {
    fn mod_name(&self) -> &str {
        &self.name
    }
}

impl State {
    /// Make the current module name one deeper than before
    /// Call this when you go deeper in the stack, and drop the state guard when you pop-back up
    pub fn new_mod(&mut self, module_name: impl ToString) -> StateGuard<'_> {
        self.mods.push_back(module_name.to_string());
        StateGuard { state: self }
    }

    /// Given a struct or enum name, returns the module where
    /// the type was declared
    pub fn mod_for_type(&self, type_name: &str) -> Option<&str> {
        self.struct_to_mod.get(type_name).map(String::as_str)
    }
}

#[derive(Debug)]
pub struct StateGuard<'a> {
    state: &'a mut State,
}

impl<'a> StateGuard<'a> {
    /// Records that a new struct/enum has been declared
    /// If we haven't gone into any modules yet, throws an error
    pub fn new_type(&mut self, type_name: String) -> Result<()> {
        let top = self.state.mods.back().ok_or_else(|| {
            Error::new(format!(
                "Tried to declare {type_name} without entering a module"
            ))
        })?;
        self.state.struct_to_mod.insert(type_name, top.clone());
        Ok(())
    }

    /// Make the current module name one deeper than before
    pub fn new_mod(&mut self, module_name: String) -> StateGuard<'_> {
        self.state.new_mod(module_name)
    }

    /// Given a struct or enum name, returns the module where
    /// the type was declared
    pub fn mod_for_type(&self, type_name: &str) -> Option<&str> {
        self.state.mod_for_type(type_name)
    }
}

impl<'a> Drop for StateGuard<'a> {
    /// When you drop a state-guard, you're going up in the stack
    fn drop(&mut self) {
        self.state.mods.pop_back();
    }
}
