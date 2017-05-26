use std::fmt::Display;

use serde_json::Value;

use vars::EMPTY;
use {DataError, Variable};

pub const TRUE: &'static &'static str = &"true";
pub const FALSE: &'static &'static str = &"false";


impl Variable for Value {
    fn attr<'x>(&'x self, attr: &str)
        -> Result<&'x Variable, DataError>
    {
        use serde_json::Value::*;
        match *self {
            Object(ref x) => {
                x.get(attr)
                .map(|x| x as &Variable)
                .ok_or(DataError::AttrNotFound)
            }
            _ => Err(DataError::AttrUnsupported(self.typename()))
        }
    }
    fn index<'x>(&'x self, key: &Variable)
        -> Result<&Variable, DataError>
    {
        use serde_json::Value::*;
        match *self {
            Object(ref x) => {
                x.get(key.as_str_key()?)
                .map(|x| x as &Variable)
                .ok_or(DataError::AttrNotFound)
            }
            Array(ref x) => {
                x.get(key.as_int_key()?)
                .map(|x| x as &Variable)
                .ok_or(DataError::IndexNotFound)
            }
            _ => Err(DataError::IndexUnsupported(self.typename()))
        }
    }
    fn output(&self) -> Result<&Display, DataError> {
        use serde_json::Value::*;
        match *self {
            Null => Ok(EMPTY),
            Bool(x) => if x { Ok(TRUE) } else { Ok(FALSE) },
            I64(ref x) => Ok(x),
            U64(ref x) => Ok(x),
            F64(ref x) => Ok(x),
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
            I64(_) => "number",
            U64(_) => "number",
            F64(_) => "number",
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
            I64(x) => Ok(x != 0),
            U64(x) => Ok(x != 0),
            F64(x) => Ok(x != 0.),
            String(ref s) => Ok(s.len() > 0),
            Array(ref a) => Ok(a.len() > 0),
            Object(ref o) => Ok(o.len() > 0),
        }
    }
}
