use std::fmt::{Display, Debug};
use std::collections::HashMap;


/// A trait that you need to implement to put variable into the rendering
/// context
pub trait Variable: Debug {
    fn attr(&self, attr: &str) -> Option<&Variable>;
    fn item(&self, key: &Variable) -> Option<&Variable>;
    fn output(&self) -> Option<&Display>;
}

/// Holds variables passed to a template rendering function
pub struct Context<'a> {
    vars: HashMap<&'a str, &'a Variable>,
}

impl<'a> Context<'a> {
    /// Create a new context
    pub fn new() -> Context<'a> {
        Context {
            vars: HashMap::new(),
        }
    }

    /// Add a variable to context
    pub fn add(&mut self, key: &'a str, value: &'a Variable) {
        self.vars.insert(key, value);
    }
}
