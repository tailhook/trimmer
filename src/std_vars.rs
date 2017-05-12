use std::fmt::Display;
use std::collections::HashMap;

use render_error::DataError;
use {Variable, Context};


impl Variable for String {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn output(&self, _: &mut Context) -> Result<&Display, DataError> {
        Ok(self)
    }
    fn as_bool(&self, _: &mut Context) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
}

impl<'a> Variable for &'a str {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn output(&self, _: &mut Context) -> Result<&Display, DataError> {
        Ok(self)
    }
    fn as_bool(&self, _: &mut Context) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
}

impl<'a> Variable for &'a String {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn output(&self, _: &mut Context) -> Result<&Display, DataError> {
        Ok(self)
    }
    fn as_bool(&self, _: &mut Context) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
}

impl<'a, V: Variable> Variable for HashMap<&'a str, V> {
    fn attr<'x>(&'x self, _: &mut Context, attr: &str)
        -> Result<&'x Variable, DataError>
    {
        self.get(attr)
        .map(|x| x as &Variable)
        .ok_or_else(|| DataError::VariableNotFound(attr.to_string()))
    }
    fn typename(&self) -> &'static str {
        "HashMap"
    }
    fn as_bool(&self, _: &mut Context) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
}

impl<T: Variable> Variable for Vec<T> {
    fn typename(&self) -> &'static str {
        "Vec"
    }
    fn as_bool(&self, _: &mut Context) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
}
