use std::rc::Rc;
use std::collections::HashMap;

use owning_ref::{ErasedRcRef, OwningRef};

use vars::{Variable, Var};
use render_error::DataError;

#[derive(Debug)]
pub struct Context<'a> {
    parent: Option<&'a Context<'a>>,
    local: HashMap<ErasedRcRef<str>, ErasedRcRef<Variable>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'static> {
        Context {
            parent: None,
            local: HashMap::new(),
        }
    }
    pub fn sub(&self) -> Context {
        Context {
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
    pub fn set<V: Variable + 'static>(&mut self, name: String, value: V) {
        self.local.insert(
            OwningRef::new(Rc::new(name))
                .map(|n| &n[..]).erase_owner(),
            OwningRef::new(Rc::new(value))
                .map(|v| v as &Variable).erase_owner());
    }
}

pub fn set(ctx: &mut Context, name: ErasedRcRef<str>,
    value: ErasedRcRef<Variable>)
{
    ctx.local.insert(name, value);
}
