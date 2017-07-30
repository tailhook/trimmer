use std::rc::Rc;
use std::mem::transmute;
use std::collections::HashMap;

use owning_ref::{ErasedRcRef, OwningRef, Erased};

use vars::{Variable, VarRef};
use render_error::DataError;


/// A set of variables passed to a template
//#[derive(Debug)]
pub struct Context<'render> {
    nothing: Rc<Erased+'render>,
    vars: HashMap<&'render str, &'render (Variable<'render> + 'render)>,
}

pub enum Parent<'a, 'render: 'a> {
    Root(&'a Context<'render>),
    Sub(&'a SubContext<'a, 'render>),
}

pub struct SubContext<'a, 'render: 'a> {
    parent: Parent<'a, 'render>,
    local: HashMap<ErasedRcRef<str>, VarRef<'render>>,
}

impl<'render> Context<'render> {
    /// Create a new empty context
    pub fn new<'x>() -> Context<'x> {
        Context {
            nothing: Rc::new(()),
            vars: HashMap::new(),
        }
    }
    /// Set the variable in the context
    pub fn set<V>(&mut self, name: &'render str, var: &'render V)
        where V: Variable<'render> + 'render
    {
        self.vars.insert(name, var);
    }
    /// Create a new context by deriving (borrowing) this context
    ///
    /// Variables can be overriden in the new context
    pub fn sub<'x>(&'x self) -> SubContext<'x, 'render> {
        SubContext {
            parent: Parent::Root(self),
            local: HashMap::new()
        }
    }
}

impl<'a, 'render: 'a> SubContext<'a, 'render> {
    /// Create a new context by deriving (borrowing) this context
    ///
    /// Variables can be overriden in the new context
    pub fn sub<'x>(&'x self) -> SubContext<'x, 'render> {
        SubContext {
            parent: Parent::Sub(self),
            local: HashMap::new()
        }
    }
}

pub fn set<'x, 'render>(ctx: &mut SubContext<'x, 'render>,
    name: ErasedRcRef<str>,
    value: VarRef<'render>)
{
    ctx.local.insert(name, value);
}

pub fn get<'x, 'render>(ctx: &SubContext<'x, 'render>, name: &str)
    -> Result<VarRef<'render>, DataError>
{
    if let Some(value) = ctx.local.get(name) {
        return Ok(value.clone());
    }
    match ctx.parent {
        Parent::Sub(ref parent) => get(parent, name),
        Parent::Root(ref root) => {
            if let Some(value) = root.vars.get(name) {
                Ok(OwningRef::new(root.nothing.clone())
                    // This looks like safe because we limit the use of
                    // the owning ref to the template render time anyway
                    // and value has lifetime of 'render
                    .map(|_| unsafe { transmute(*value) }))
            } else {
                Err(DataError::VariableNotFound(name.to_string()))
            }
        }
    }
}
