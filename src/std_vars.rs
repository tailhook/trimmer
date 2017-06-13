use std::fmt::Display;
use std::collections::HashMap;
use std::slice::Iter;

use render_error::DataError;
use vars::{Variable, Var};

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
    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=Var<'x>> + 'x>, DataError>
    {

        struct VecIter<'a, T: 'a>(Iter<'a, T>);

        impl<'a, T: Variable + 'static> Iterator for VecIter<'a, T> {
            type Item = Var<'a>;
            fn next(&mut self) -> Option<Var<'a>> {
                self.0.next().map(|x| Var::Ref(x as &Variable))
            }
        }

        Ok(Box::new(VecIter(self.iter())))
    }
}
