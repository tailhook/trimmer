use std::rc::Rc;
use std::collections::HashMap;

use owning_ref::{ErasedRcRef, OwningRef};

use vars::{Variable, Var};
use render_error::DataError;

#[derive(Debug)]
enum Parent<'a> {
    Root(Rc<Box<Variable + 'static>>),
    Map(&'a Varmap<'a>),
}

#[derive(Debug)]
pub struct Varmap<'a> {
    parent: Parent<'a>,
    local: HashMap<ErasedRcRef<str>, ErasedRcRef<Variable>>,
}

impl<'a> Varmap<'a> {
    pub fn new<V: Variable + 'static>(root: V) -> Varmap<'static> {
        Varmap {
            parent: Parent::Root(Rc::new(Box::new(root) as Box<Variable>)),
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
            return Ok(value.clone());
        }
        match self.parent {
            Parent::Root(ref var) => {
                OwningRef::new(var.clone())
                    .map(|v| &**v)
                    .erase_owner();
                    /*
                    .try_map(|v: &(Variable + 'static)| {
                        match v.attr(name) {
                            Ok(Var::Ref(ref x)) => Ok(*x),
                            Ok(Var::Rc(x)) => Err(Ok(x)),
                            Err(e) => Err(Err(e)),
                        }
                    }
                );*/
                unimplemented!();
                /* {
                    Ok(x) => Ok(x.erase_owner()),
                    Err(Ok(x)) => Ok(x),
                    Err(Err(e)) => Err(e),
                }*/
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
