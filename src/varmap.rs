use std::collections::HashMap;

use {Variable, Context};
use render_error::DataError;


#[derive(Debug)]
pub struct Varmap<'a, 'b> {
    parent: &'b Variable,
    local: HashMap<String, &'a Variable>,
}

impl<'a, 'b> Varmap<'a, 'b> {
    pub fn new<'x>(parent: &'x Variable) -> Varmap<'x, 'x> {
        Varmap {
            parent: parent,
            local: HashMap::new(),
        }
    }
}

impl<'a, 'b> Variable for Varmap<'a, 'b> {
    fn attr<'x>(&'x self, ctx: &mut Context, attr: &str)
        -> Result<&'x Variable, DataError>
    {
        if let Some(value) = self.local.get(attr) {
            return Ok(*value);
        }
        return self.parent.attr(ctx, attr);
    }
    fn typename(&self) -> &'static str {
        "Varmap"
    }
}
