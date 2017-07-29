use std::fmt::Display;
use std::collections::HashMap;
use std::slice::Iter;

use render_error::DataError;
use vars::{Variable, Var};


impl<'a> Variable for &'a Variable {
    fn typename(&self) -> &'static str {
        (*self).typename()
    }
}

impl Variable for &'static str {
    fn typename(&self) -> &'static str {
        (*self).typename()
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
    fn as_str_key(&self) -> Result<&str, DataError> {
        Ok(self)
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
}

impl Variable for String {
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

impl Variable for u16 {
    fn typename(&self) -> &'static str {
        "u16"
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(*self != 0)
    }
}

impl<V: Variable + 'static> Variable for HashMap<String, V> {
    fn attr(&self, attr: &str)
        -> Result<Var, DataError>
    {
        self.get(attr)
        .map(|x| Var::borrow(x))
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
    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=Var<'x>>+'x>, DataError>
    {
        Ok(Box::new(self.iter().map(|x| Var::borrow(x))))
    }
}
