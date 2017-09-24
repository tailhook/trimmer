use std::iter::empty;
use std::usize;

use serde_json::Value;

use {DataError, Variable, Var, Context, Template, RenderError, Pos, Output};
use {Number};
use compare::Comparable;

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
            if val.as_u64().map(|x| x <= usize::MAX as u64).unwrap_or(false)
            => Ok(val.as_u64().unwrap() as usize),
            // TODO(tailhook) try use float too
            // TODO(tailhook) show out of range int error
            _ => Err(DataError::IntKeyUnsupported(self.typename())),
        }
    }
    fn as_number(&self) -> Result<Number, DataError> {
        use serde_json::Value::*;
        match *self {
            Number(ref val) if val.is_u64() => {
                Ok(val.as_u64().unwrap().into())
            }
            Number(ref val) if val.is_i64() => {
                Ok(val.as_i64().unwrap().into())
            }
            Number(ref val) if val.is_f64() => {
                Ok(val.as_f64().unwrap().into())
            }
            _ => Err(DataError::NumberUnsupported(self.typename())),
        }
    }
    fn as_comparable(&self) -> Result<Comparable, DataError> {
        use serde_json::Value::*;
        match *self {
            Bool(x) => Ok(x.into()),
            Number(ref val) if val.is_u64() => {
                Ok(val.as_u64().unwrap().into())
            }
            Number(ref val) if val.is_i64() => {
                Ok(val.as_i64().unwrap().into())
            }
            Number(ref val) => {
                Ok(val.as_f64().unwrap().into())
            }
            String(ref s) => Ok(s[..].into()),
            Array(_) => Err(DataError::ComparisonUnsupported(self.typename())),
            Object(_) => Err(DataError::ComparisonUnsupported(self.typename())),
            Null => Err(DataError::ComparisonUnsupported(self.typename())),
        }
    }
    fn output(&self) -> Result<Output, DataError> {
        use serde_json::Value::*;
        match *self {
            Null => Ok(Output::empty()),
            Bool(x) => if x { Ok(TRUE.into()) } else { Ok(FALSE.into()) },
            Number(ref x) => Ok(x.into()),
            String(ref s) => Ok(s.into()),
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

/// Renders a template using variables from `serde_json::Value`
///
/// The value must be JSON object or error is returned
pub fn render_json(tpl: &Template, json: &Value) -> Result<String, RenderError>
{
    match json.as_object() {
        Some(map) => {
            let mut ctx = Context::new();
            for (k, v) in map {
                ctx.set(k, v);
            }
            tpl.render(&ctx)
        }
        None => {
            Err(RenderError::Data(vec![
                (Pos { line: 0, column: 0 },
                 DataError::Custom("render_json expects a JSON object".into()))
            ]))
        }
    }
}
