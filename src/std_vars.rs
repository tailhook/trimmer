use std::fmt::Display;
use std::collections::HashMap;
use std::slice::Iter;

use target::{Target, TargetKind};

use render_error::DataError;
use {Variable, Context, Iterator};

struct VecIterator<'a, T: Variable + 'a> {
    vec: &'a Vec<T>,
    index: usize,
}


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

impl<'a, T: Variable + 'a> Variable for Vec<T> {
    fn typename(&self) -> &'static str {
        "Vec"
    }
    fn as_bool(&self, _: &mut Context) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn iterate<'x>(&'x self, _ctx: &mut Context, target: TargetKind)
        -> Result<Box<Iterator<'x>+'x>, DataError>
    {
        Ok(Box::new(VecIterator { vec: self, index: 0 }))
    }
}

impl<'a, T: Variable + 'a> Iterator<'a> for VecIterator<'a, T> {
    fn next<'y, 'z>(&mut self, _ctx: &mut Context<'a>,
        target: &mut Target<'a, 'y, 'z>)
        -> bool
    {
        match self.vec.get(self.index) {
            Some(x) => {
                target.set(x);
                self.index += 1;
                true
            },
            None => false,
        }
    }
}
