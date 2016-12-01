use std::fmt::Display;
use std::collections::HashMap;

use render_error::DataError;
use {Variable};


impl Variable for String {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
}

impl<'a> Variable for &'a str {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
}

impl<'a> Variable for &'a String {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
}

impl<'a, V: Variable> Variable for HashMap<&'a str, V> {
    fn attr<'x>(&'x self, attr: &str) -> Result<&'x Variable, DataError> {
        self.get(attr)
        .map(|x| x as &Variable)
        .ok_or_else(|| DataError::VariableNotFound(attr.to_string()))
    }
    fn typename(&self) -> &'static str {
        "HashMap"
    }
}
