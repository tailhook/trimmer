use std::fmt::{self, Display, Debug};
use std::collections::HashMap;

use render_error::DataError;

#[derive(Debug)]
pub struct Undefined;

pub const UNDEFINED: &'static Undefined = &Undefined;
const EMPTY: &'static &'static str = &"";

pub struct Var<'a> {
    value: VarImpl<'a>
}

pub enum VarImpl<'a> {
    Str(&'a str),
    Borrowed(&'a Variable),
    Owned(Box<Variable+'a>),
}

/// A trait that you need to implement to put variable into the rendering
/// context
///
/// Note: by default all operations return unsupported error
pub trait Variable: Debug {
    /// Evaluates `a.b` operation
    ///
    /// Depending on your domain `a.x` may be equivalent of `a["x"]` or
    /// maybe not. Integer arguments `a.0` are not supported.
    fn attr<'x>(&'x self, _attr: &str) -> Result<Var<'x>, DataError> {
        Err(DataError::AttrUnsupported(self.typename()))
    }
    /// Evaluates `a[b]`
    ///
    /// Depending on your domain `a["x"]` may be equivalent of `a.x` or
    /// maybe not.
    ///
    /// You may exract string value for a key with `key.output().to_string()`.
    /// Note that actual key may have a (rust) type that is different from
    /// type of self (i.e. may come from different library).
    fn subscript(&self, _key: &Variable) -> Result<Var, DataError> {
        Err(DataError::SubscriptUnsupported(self.typename()))
    }
    /// Evaluates `{{ x }}`
    ///
    /// This operation may not be useful for array-, and mapping-like values
    fn output(&self) -> Result<&Display, DataError> {
        Err(DataError::OutputUnsupported(self.typename()))
    }
    /// Returns type name to use in error messages
    ///
    /// Note this must return actual type of value from user point of view
    /// not just rust type. For example for `Json` type it should describe
    /// `Json::Object` as a mapping and `Json::Array` as an array, not just
    /// return `Json`
    fn typename(&self) -> &'static str;
}

/// Holds variables passed to a template rendering function
pub struct Context<'a> {
    vars: HashMap<&'a str, Var<'a>>,
}

impl<'a> Context<'a> {
    /// Create a new context
    pub fn new() -> Context<'a> {
        Context {
            vars: HashMap::new(),
        }
    }

    /// Add a variable to context
    pub fn add<V: IntoVariable<'a> + Debug + 'a>(&mut self,
        key: &'a str, value: V)
    {
        let v= value.into_variable();
        println!("ADD {:?} = {:?}", key, v);
        self.vars.insert(key, v);
    }

    /// Context
    pub fn get(&'a self, key: &'a str) -> Option<Var<'a>> {
        self.vars.get(key).map(IntoVariable::into_variable)
    }
}

impl Variable for Undefined {
    fn attr<'x>(&'x self, _attr: &str) -> Result<Var<'x>, DataError> {
        Ok(Var { value: VarImpl::Borrowed(UNDEFINED) })
    }
    fn subscript<'x>(&'x self, _key: &Variable) -> Result<Var<'x>, DataError> {
        Ok(Var { value: VarImpl::Borrowed(UNDEFINED) })
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(EMPTY)
    }
    fn typename(&self) -> &'static str {
        "undefined"
    }
}

pub fn undefined() -> Var<'static> {
    Var { value: VarImpl::Borrowed(UNDEFINED) }
}

impl<'a> Debug for Var<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            VarImpl::Borrowed(x) => write!(f, "b{:?}", x),
            VarImpl::Owned(ref x) => write!(f, "o{:?}", x),
            VarImpl::Str(x) => write!(f, "s{:?}", x),
        }
    }
}

impl<'a> Variable for Var<'a> {
    fn attr<'x>(&'x self, attr: &str) -> Result<Var<'x>, DataError> {
        match self.value {
            VarImpl::Borrowed(x) => x.attr(attr),
            VarImpl::Owned(ref x) => x.attr(attr),
            VarImpl::Str(_) => Err(DataError::AttrUnsupported("&str")),
        }
    }
    fn subscript<'x>(&'x self, key: &Variable) -> Result<Var<'x>, DataError> {
        match self.value {
            VarImpl::Borrowed(x) => x.subscript(key),
            VarImpl::Owned(ref x) => x.subscript(key),
            VarImpl::Str(_) => Err(DataError::SubscriptUnsupported("&str")),
        }
    }
    fn output(&self) -> Result<&Display, DataError> {
        match self.value {
            VarImpl::Borrowed(x) => x.output(),
            VarImpl::Owned(ref x) => x.output(),
            VarImpl::Str(ref x) => Ok(x),
        }
    }
    fn typename(&self) -> &'static str {
        match self.value {
            VarImpl::Borrowed(x) => x.typename(),
            VarImpl::Owned(ref x) => x.typename(),
            VarImpl::Str(_) => "&str",
        }
    }
}


pub trait IntoVariable<'a> {
    fn into_variable(self) -> Var<'a>;
}


impl<'a> IntoVariable<'a> for String {
    fn into_variable(self) -> Var<'a> {
        Var { value: VarImpl::Owned(Box::new(self)) }
    }
}

impl<'a> IntoVariable<'a> for &'a str {
    fn into_variable(self) -> Var<'a> {
        Var { value: VarImpl::Str(self) }
    }
}

impl<'a, T: Variable + 'a> IntoVariable<'a> for &'a T
{
    fn into_variable(self) -> Var<'a> {
        Var { value: VarImpl::Borrowed(self) }
    }
}

