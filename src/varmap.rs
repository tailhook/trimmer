use std::rc::Rc;
use std::collections::HashMap;

use owning_ref::{ErasedRcRef, OwningRef};

use vars::{Variable, Var};
use render_error::DataError;

#[derive(Debug)]
pub struct Varmap<'a> {
    parent: Option<&'a Varmap<'a>>,
    local: HashMap<ErasedRcRef<str>, ErasedRcRef<Variable>>,
}

impl<'a> Varmap<'a> {
    pub fn new() -> Varmap<'static> {
        Varmap {
            parent: None,
            local: HashMap::new(),
        }
    }
    pub fn sub(&self) -> Varmap {
        Varmap {
            parent: Some(self),
            local: HashMap::new()
        }
    }
    pub fn get(&self, name: &str)
        -> Result<ErasedRcRef<Variable>, DataError>
    {
        if let Some(value) = self.local.get(name) {
            return Ok(value.clone());
        }
        match self.parent {
            Some(ref parent) => parent.get(name),
            None => Err(DataError::VariableNotFound(name.to_string())),
        }
    }
    pub fn set(&mut self, name: ErasedRcRef<str>,
                          value: ErasedRcRef<Variable>)
    {
        self.local.insert(name, value);
    }
}
