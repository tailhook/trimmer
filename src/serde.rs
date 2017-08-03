use std::fmt::Display;
use std::iter::empty;
use std::usize;

use serde_json::Value;

use vars::{EMPTY_STR};
use {DataError, Variable, Var};

pub const TRUE: &'static &'static str = &"true";
pub const FALSE: &'static &'static str = &"false";


impl<'render> Variable<'render> for Value {
    fn attr<'x>(&'x self, attr: &str)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        use serde_json::Value::*;
        match *self {
            Object(ref x) => {
                x.get(attr)
                .map(|x| Var::borrow(x))
                .ok_or(DataError::AttrNotFound)
            }
            _ => Err(DataError::AttrUnsupported(self.typename()))
        }
    }
    fn index<'x>(&'x self, key: &Variable<'render>)
        -> Result<Var<'x, 'render>, DataError>
        where 'render: 'x
    {
        use serde_json::Value::*;
        match *self {
            Object(ref x) => {
                x.get(key.as_str_key()?)
                .map(|x| Var::borrow(x))
                .ok_or(DataError::AttrNotFound)
            }
            Array(ref x) => {
                x.get(key.as_int_key()?)
                .map(|x| Var::borrow(x))
                .ok_or(DataError::IndexNotFound)
            }
            _ => Err(DataError::IndexUnsupported(self.typename()))
        }
    }
    fn as_str_key<'x>(&'x self) -> Result<&'x str, DataError> {
        use serde_json::Value::*;
        match *self {
            String(ref s) => Ok(s),
            _ => Err(DataError::StrKeyUnsupported(self.typename())),
        }
    }
    fn as_int_key(&self) -> Result<usize, DataError> {
        use serde_json::Value::*;
        match *self {
            Number(ref val)
            if val.as_u64() .map(|x| x <= usize::MAX as u64).unwrap_or(false)
            => Ok(val.as_u64().unwrap() as usize),
            // TODO(tailhook) try use float too
            // TODO(tailhook) show out of range int error
            _ => Err(DataError::IntKeyUnsupported(self.typename())),
        }
    }
    fn output(&self) -> Result<&Display, DataError> {
        use serde_json::Value::*;
        match *self {
            Null => Ok(EMPTY_STR),
            Bool(x) => if x { Ok(TRUE) } else { Ok(FALSE) },
            Number(ref x) => Ok(x),
            String(ref s) => Ok(s),
            Array(_) => Err(DataError::OutputUnsupported(self.typename())),
            Object(_) => Err(DataError::OutputUnsupported(self.typename())),
        }
    }
    fn typename(&self) -> &'static str {
        use serde_json::Value::*;
        match *self {
            Null => "null",
            Bool(_) => "bool",
            Number(_) => "number",
            String(_) => "string",
            Array(_) => "array",
            Object(_) => "object",
        }
    }
    fn as_bool(&self) -> Result<bool, DataError> {
        use serde_json::Value::*;
        match *self {
            Null => Ok(false),
            Bool(x) => Ok(x),
            Number(ref x) => Ok(x.as_u64().map(|x| x != 0).unwrap_or(true)),
            String(ref s) => Ok(s.len() > 0),
            Array(ref a) => Ok(a.len() > 0),
            Object(ref o) => Ok(o.len() > 0),
        }
    }
    fn iterate<'x>(&'x self)
        -> Result<Box<Iterator<Item=Var<'x, 'render>>+'x>, DataError>
        where 'render: 'x
    {
        use serde_json::Value::*;
        match *self {
            Null => Ok(Box::new(empty())),
            Bool(..) | Number(..) | String(..) | Object(..)
            => Err(DataError::IterationUnsupported(self.typename())),
            Array(ref a) => Ok(Box::new(
                a.iter().map(|x| Var::borrow(x)))),
        }
    }
    fn iterate_pairs<'x>(&'x self)
        -> Result<Box<Iterator<Item=(Var<'x, 'render>, Var<'x, 'render>)>+'x>,
                  DataError>
        where 'render: 'x
    {
        use serde_json::Value::*;
        match *self {
            Null => Ok(Box::new(empty())),
            Bool(..) | Number(..) | String(..) | Array(..)
            => Err(DataError::IterationUnsupported(self.typename())),
            Object(ref o) => Ok(Box::new(
                o.iter().map(|(k, v)| (Var::borrow(k), Var::borrow(v))))),
        }
    }
}
