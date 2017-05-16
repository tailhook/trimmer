use std::collections::HashMap;

use {Variable, Context};
use render_error::DataError;


#[derive(Debug)]
pub struct Varmap<'a: 'b, 'b> { // TODO(tailhook) enum?
    root: &'a Variable,
    parent: Option<&'b Varmap<'a, 'b>>,
    local: HashMap<String, &'a Variable>,
}

impl<'a, 'b> Varmap<'a, 'b> {
    pub fn new(root: &'a Variable) -> Varmap<'a, 'b> {
        Varmap {
            root,
            parent: None,
            local: HashMap::new(),
        }
    }
    pub fn sub(&'b self) -> Varmap<'a, 'b> {
        Varmap {
            root: self.root,
            parent: self.parent,
            local: HashMap::new()
        }
    }
    pub fn get(&self, ctx: &mut Context, name: &str)
        -> Result<&'a Variable, DataError>
    {
        if let Some(value) = self.local.get(name) {
            return Ok(*value);
        }
        if let Some(ref parent) = self.parent {
            parent.get(ctx, name)
        } else {
            self.root.attr(ctx, name)
        }
    }
    pub fn set(&mut self, name: String, value: &'a Variable) {
        self.local.insert(name, value);
    }
}
