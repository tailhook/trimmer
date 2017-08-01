use std::fmt::Display;
use std::collections::HashMap;

use render_error::DataError;
use vars::{Variable};
use {Var};


impl<'a, 'render: 'a> Variable<'render> for &'a str {
    fn typename(&self) -> &'static str {
        "str"
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

impl<'x> Variable<'x> for String {
    fn typename(&self) -> &'static str {
        "String"
    }
    fn as_str_key(&self) -> Result<&str, DataError> {
        Ok(&self[..])
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
}

impl<'x> Variable<'x> for u16 {
    fn typename(&self) -> &'static str {
        "u16"
    }
    fn as_int_key(&self) -> Result<usize, DataError> {
        Ok(*self as usize)
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(self)
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(*self != 0)
    }
}

impl<'render, V> Variable<'render> for HashMap<String, V>
    where V: Variable<'render> + 'render
{
    fn attr<'x>(&'x self, attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(attr)
        .map(|x| Var::borrow(x))
        .ok_or_else(|| DataError::VariableNotFound(attr.to_string()))
    }
    fn index<'x>(&'x self, index: &(Variable<'render>+'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(index.as_str_key()?)
        .map(|x| Var::borrow(x))
        .ok_or(DataError::IndexNotFound)
    }
    fn typename(&self) -> &'static str {
        "HashMap"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
}

impl<'a: 'render, 'render, V> Variable<'render> for HashMap<&'a str, V>
    where V: Variable<'render> + 'render
{
    fn attr<'x>(&'x self, attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(attr)
        .map(|x| Var::borrow(x))
        .ok_or_else(|| DataError::VariableNotFound(attr.to_string()))
    }
    fn index<'x>(&'x self, index: &(Variable<'render>+'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(index.as_str_key()?)
        .map(|x| Var::borrow(x))
        .ok_or(DataError::IndexNotFound)
    }
    fn typename(&self) -> &'static str {
        "HashMap"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'render>, Var<'x, 'render>)>+'x>,
                  DataError>
        where 'render: 'x
    {
        Ok(Box::new(self.iter()
            .map(|(x, y)| (Var::borrow(x), Var::borrow(y)))))
    }
}



impl<'a, 'render, T: Variable<'render> + 'render> Variable<'render> for Vec<T> {
    fn typename(&self) -> &'static str {
        "Vec"
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        Ok(self.len() > 0)
    }
    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=Var<'x, 'render>>+'x>, DataError>
        where 'render: 'x
    {
        Ok(Box::new(self.iter().map(|x| Var::borrow(x))))
    }
    fn index<'x>(&'x self, index: &(Variable<'render>+'render))
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        self.get(index.as_int_key()?)
        .map(|x| Var::borrow(x))
        .ok_or(DataError::IndexNotFound)
    }
}
