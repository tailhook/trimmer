use std::fmt::{self, Display, Debug};

use render_error::DataError;

#[derive(Debug)]
pub struct Undefined;

pub const UNDEFINED: &'static Undefined = &Undefined;
pub const EMPTY: &'static &'static str = &"";

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
    fn attr<'x>(&'x self, _attr: &str) -> Result<&'x Variable, DataError> {
        Err(DataError::AttrUnsupported(self.typename()))
    }
    /// Evaluates `a[b]`
    ///
    /// Depending on your domain `a["x"]` may be equivalent of `a.x` or
    /// maybe not.
    ///
    /// You may exract string value for a key with `key.as_str_key()`
    /// and `key.as_int_key()`.
    ///
    /// Note that actual key may have a (rust) type that is different from
    /// type of self (i.e. may come from different library).
    fn index(&self, _key: &Variable) -> Result<Var, DataError> {
        Err(DataError::IndexUnsupported(self.typename()))
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
    /// Return string value of the variable used as key in index
    ///
    /// String keys are used for indexing dicts
    ///
    /// It's okay not to implement this method for complex variables
    fn as_str_key(&self) -> Result<&str, DataError> {
        Err(DataError::StrKeyUnsupported(self.typename()))
    }
    /// Return intenger value of the variable used as key
    ///
    /// Integer keys are used for indexing arrays
    ///
    /// It's okay not to implement this method for complex variables
    fn as_int_key(&self) -> Result<usize, DataError> {
        Err(DataError::IntKeyUnsupported(self.typename()))
    }
}

impl Variable for Undefined {
    fn attr<'x>(&'x self, _attr: &str) -> Result<&'x Variable, DataError> {
        Ok(UNDEFINED)
    }
    fn index<'x>(&'x self, _key: &Variable) -> Result<Var<'x>, DataError> {
        Ok(Var { value: VarImpl::Borrowed(UNDEFINED) })
    }
    fn output(&self) -> Result<&Display, DataError> {
        Ok(EMPTY)
    }
    fn typename(&self) -> &'static str {
        "undefined"
    }
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

/*
impl<'a> Variable for Var<'a> {
    fn attr<'x>(&'x self, attr: &str) -> Result<Var<'x>, DataError> {
        match self.value {
            VarImpl::Borrowed(x) => x.attr(attr),
            VarImpl::Owned(ref x) => x.attr(attr),
            VarImpl::Str(_) => Err(DataError::AttrUnsupported("&str")),
        }
    }
    fn index<'x>(&'x self, key: &Variable) -> Result<Var<'x>, DataError> {
        match self.value {
            VarImpl::Borrowed(x) => x.index(key),
            VarImpl::Owned(ref x) => x.index(key),
            VarImpl::Str(_) => Err(DataError::IndexUnsupported("&str")),
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
*/


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

