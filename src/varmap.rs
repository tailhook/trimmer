use std::rc::Rc;
use std::collections::HashMap;

use owning_ref::{ErasedRcRef, OwningRef};

use vars::{Variable, VarRef};
use render_error::DataError;


/// A set of variables passed to a template
///
/// You can create a new context by inheriting and overriding variables.
/// See ``sub()`` method.
//#[derive(Debug)]
pub struct Context<'a, 'render: 'a> {
    parent: Option<&'a Context<'a, 'render>>,
    local: HashMap<ErasedRcRef<str>, VarRef<'render>>,
}

impl<'a, 'render: 'a> Context<'a, 'render> {
    /// Create a new empty context
    pub fn new<'x, 'y>() -> Context<'x, 'x> {
        Context {
            parent: None,
            local: HashMap::new(),
        }
    }
    /// Create a new context by deriving (borrowing) this context
    ///
    /// Variables can be overriden in the new context
    pub fn sub<'x>(&'x self) -> Context<'x, 'render> {
        Context {
            parent: Some(self),
            local: HashMap::new()
        }
    }
    /// Get variable from a context and fallback to parent context if not found
    ///
    pub fn get(&self, name: &str) -> Option<&Variable<'render>> {
        if let Some(value) = self.local.get(name) {
            return Some(&**value);
        }
        self.parent.and_then(|p| p.get(name))
    }
    /// Set the variable in context
    pub fn set<V>(&mut self, name: String, value: V)
        where V: Variable<'render> + 'render
    {
        self.local.insert(
            OwningRef::new(Rc::new(name))
                .map(|n| &n[..]).erase_owner(),
            OwningRef::new(Rc::new(value))
                .map(|v| v as &Variable).erase_owner());
    }
}

pub fn set<'x, 'render>(ctx: &mut Context<'x, 'render>, name: ErasedRcRef<str>,
    value: VarRef<'render>)
{
    ctx.local.insert(name, value);
}

pub fn get<'x, 'render>(ctx: &Context<'x, 'render>, name: &str)
    -> Result<VarRef<'render>, DataError>
{
    if let Some(value) = ctx.local.get(name) {
        return Ok(value.clone());
    }
    match ctx.parent {
        Some(ref parent) => get(parent, name),
        None => Err(DataError::VariableNotFound(name.to_string())),
    }
}
