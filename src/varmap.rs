use std::rc::Rc;
use std::collections::HashMap;

use owning_ref::{ErasedRcRef, OwningRef};

use vars::{Variable, Var};
use render_error::DataError;

/// A set of variables passed to a template
///
/// You can create a new context by inheriting and overriding variables.
/// See ``sub()`` method.
#[derive(Debug)]
pub struct Context<'a> {
    parent: Option<&'a Context<'a>>,
    local: HashMap<ErasedRcRef<str>, ErasedRcRef<Variable>>,
}

impl<'a> Context<'a> {
    /// Create a new empty context
    pub fn new() -> Context<'static> {
        Context {
            parent: None,
            local: HashMap::new(),
        }
    }
    /// Create a new context by deriving (borrowing) this context
    ///
    /// Variables can be overriden in the new context
    pub fn sub(&self) -> Context {
        Context {
            parent: Some(self),
            local: HashMap::new()
        }
    }
    /// Get variable from a context and fallback to parent context if not found
    ///
    pub fn get(&self, name: &str) -> Option<&Variable> {
        if let Some(value) = self.local.get(name) {
            return Some(&**value);
        }
        self.parent.and_then(|p| p.get(name))
    }
    /// Set the variable in context
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

pub fn get(ctx: &Context, name: &str)
    -> Result<ErasedRcRef<Variable>, DataError>
{
    if let Some(value) = ctx.local.get(name) {
        return Ok(value.clone());
    }
    match ctx.parent {
        Some(ref parent) => get(parent, name),
        None => Err(DataError::VariableNotFound(name.to_string())),
    }
}
