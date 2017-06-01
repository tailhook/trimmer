use std::fmt::Display;
use std::collections::HashMap;
use std::slice::Iter;

use target::{Target, TargetKind};

use render_error::DataError;
use vars::{Variable, Var, Iterator};


impl<'a> Variable for &'static str {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
}

impl<'a> Variable for String {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
}

impl<V: Variable + 'static> Variable for HashMap<String, V> {
    fn attr(&self, attr: &str)
        -> Result<Var, DataError>
    {
        self.get(attr)
        .map(|x| Var::Ref(x as &Variable))
        .ok_or_else(|| DataError::VariableNotFound(attr.to_string()))
    }
    fn typename(&self) -> &'static str {
        "HashMap"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
}

impl<'a, T: Variable + 'static> Variable for Vec<T> {
    fn typename(&self) -> &'static str {
        "Vec"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn iterate<'x>(&'x self, target: TargetKind)
        -> Result<Box<Iterator<'x> + 'x>, DataError>
    {
        Ok(Box::new(self.iter()))
    }
}

impl<'a, T: Variable + 'a> Iterator<'a> for Iter<'a, T> {
    fn next(&mut self, target: &mut Target)
        -> bool
    {
        match ::std::iter::Iterator::next(self) {
            Some(x) => {
                target.set(x);
                true
            },
            None => false,
        }
    }
}
