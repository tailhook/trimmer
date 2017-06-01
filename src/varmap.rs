use std::rc::Rc;
use std::collections::HashMap;

use {Variable};
use render_error::DataError;
use owning_ref::ErasedRcRef;

enum Parent<'a> {
    Root(Rc<Variable>),
    Map(&'a Varmap<'a>),
}

#[derive(Debug)]
pub struct Varmap<'a> {
    parent: Parent<'a>,
    local: HashMap<ErasedRcRef<str>, ErasedRcRef<Variable>>,
}

impl<'a> Varmap<'a> {
    pub fn new(root: &Rc<Variable>) -> Varmap<'static> {
        Varmap {
            parent: Parent::Root(root.clone()),
            local: HashMap::new(),
        }
    }
    pub fn sub(&self) -> Varmap {
        Varmap {
            parent: Parent::Map(self),
            local: HashMap::new()
        }
    }
    pub fn get(&self, name: &str)
        -> Result<ErasedRcRef<Variable>, DataError>
    {
        if let Some(value) = self.local.get(name) {
            return Ok(*value);
        }
        match self.parent {
            Parent::Root(ref var) => {
                var.attr(name)
            },
            Parent::Map(ref map) => {
                map.get(name)
            },
        }
    }
    pub fn set(&mut self, name: ErasedRcRef<str>,
                          value: ErasedRcRef<Variable>)
    {
        self.local.insert(name, value);
    }
}
